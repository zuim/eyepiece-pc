use super::{
    Input, MAX_FRAME, Request, Response, Startup, WebError, WebOptions, decode_request, request, response,
    response_envelope, run_worker, wire,
};
use axum::{
    Json, Router,
    body::Body,
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, Request as HttpRequest, StatusCode, Uri, header},
    middleware::{self, Next},
    response::{IntoResponse, Response as HttpResponse},
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    future::IntoFuture,
    net::{Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc,
    },
    thread,
};
use subtle::ConstantTimeEq;
use tokio::sync::{Notify, broadcast, mpsc as async_mpsc};
use tower_http::services::ServeDir;

const CLIENT_QUEUE: usize = 16;
const REQUEST_QUEUE: usize = 16;

/// Sites that may see whether eyepiece-pc runs (`/api/hello`): tinkerglass's web app, its pull
/// request previews and its local development server. `EYEPIECE_DISCOVERY_ORIGINS` adds more
/// (comma-separated origins).
const DISCOVERY_ORIGINS: &[&str] = &["https://tinkerglass-org.github.io"];
const DISCOVERY_ORIGIN_SUFFIXES: &[&str] = &[".tinkerglass-prs.pages.dev"];

#[derive(Clone)]
struct App {
    token: Arc<str>,
    hosts: Arc<HashSet<String>>,
    origins: Arc<HashSet<String>>,
    discovery_origins: Arc<HashSet<String>>,
    url: Arc<str>,
    requests: async_mpsc::Sender<ClientInput>,
    events: broadcast::Sender<Response>,
    previews: broadcast::Sender<Response>,
    next_client: Arc<AtomicU64>,
    picking: Arc<AtomicBool>,
    shutdown: Arc<Notify>,
}

enum ClientInput {
    Request(u64, Request),
    Disconnected(u64),
}

#[derive(Deserialize)]
struct Credentials {
    token: String,
}

fn origin(value: &str) -> Result<(String, String), String> {
    let uri: Uri = value
        .parse()
        .map_err(|e| format!("invalid development origin: {e}"))?;
    if uri.scheme_str() != Some("http")
        || !matches!(uri.host(), Some("localhost" | "127.0.0.1"))
        || !matches!(
            uri.path_and_query().map(|p| p.as_str()),
            None | Some("/") | Some("")
        )
        || uri.authority().is_none()
    {
        return Err("EYEP_DEV_ORIGIN must be a localhost HTTP origin".into());
    }
    let authority = uri.authority().unwrap().as_str().to_string();
    Ok((format!("http://{authority}"), authority))
}

/// The static web app: EYEPIECE_WEB_DIR if set; otherwise `web/` next to the executable (zip,
/// tar.gz and AppImage packages) or in the bundle's
/// `Contents/Resources` (macOS), then `web/build` of the checkout this binary was built from
/// (development builds).
fn web_dir() -> Result<PathBuf, String> {
    if let Some(dir) = std::env::var_os("EYEPIECE_WEB_DIR").map(PathBuf::from) {
        return if dir.is_dir() { Ok(dir) } else { Err(format!("EYEPIECE_WEB_DIR {} does not exist", dir.display())) };
    }
    let exe_dir = std::env::current_exe().ok().and_then(|exe| exe.parent().map(Path::to_path_buf));
    let packaged = exe_dir.iter().flat_map(|dir| [dir.join("web"), dir.join("../Resources/web")]);
    let checkout = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../web/build");
    packaged
        .chain([checkout.clone()])
        .find(|dir| dir.join("index.html").is_file())
        .ok_or_else(|| {
            format!(
                "the web UI is not built: run `pnpm install && pnpm build` in {} (or set EYEPIECE_WEB_DIR)",
                checkout.parent().unwrap().display()
            )
        })
}

fn configured_port() -> Result<u16, String> {
    match std::env::var("EYEPIECE_PORT") {
        Ok(port) => port
            .parse::<u16>()
            .ok()
            .filter(|port| *port != 0)
            .ok_or_else(|| "invalid EYEPIECE_PORT".into()),
        Err(std::env::VarError::NotPresent) => Ok(4174),
        Err(error) => Err(format!("invalid EYEPIECE_PORT: {error}")),
    }
}

fn allowed_header(headers: &HeaderMap, name: header::HeaderName, values: &HashSet<String>) -> bool {
    let mut entries = headers.get_all(name).iter();
    let Some(value) = entries.next() else {
        return false;
    };
    entries.next().is_none()
        && value
            .to_str()
            .ok()
            .is_some_and(|value| values.contains(value))
}

async fn protect(State(app): State<App>, request: HttpRequest<Body>, next: Next) -> HttpResponse {
    let host_ok = allowed_header(request.headers(), header::HOST, &app.hosts);
    let method_ok = matches!(
        *request.method(),
        axum::http::Method::GET | axum::http::Method::HEAD
    ) || (request.method() == axum::http::Method::OPTIONS && request.uri().path() == "/api/hello");
    if !host_ok {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }
    if !method_ok {
        return (StatusCode::METHOD_NOT_ALLOWED, "Method not allowed").into_response();
    }
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::X_CONTENT_TYPE_OPTIONS,
        header::HeaderValue::from_static("nosniff"),
    );
    response
}

async fn session(State(app): State<App>, headers: HeaderMap) -> HttpResponse {
    if (headers.contains_key(header::ORIGIN)
        && !allowed_header(&headers, header::ORIGIN, &app.origins))
        || headers
            .get("sec-fetch-site")
            .is_some_and(|value| value == "cross-site")
    {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }
    (
        [(header::CACHE_CONTROL, "no-store")],
        Json(serde_json::json!({ "token": app.token.as_ref(), "workerStatus": "running" })),
    )
        .into_response()
}

/// Whether a site may read `/api/hello`: the fixed tinkerglass origins, any loopback HTTP origin
/// (tinkerglass's `./tg web`) and the configured extra origins.
fn discovery_origin_allowed(origin: &str, extra: &HashSet<String>) -> bool {
    if DISCOVERY_ORIGINS.contains(&origin) || extra.contains(origin) {
        return true;
    }
    let Ok(uri) = origin.parse::<Uri>() else { return false };
    let bare = matches!(uri.path_and_query().map(|p| p.as_str()), None | Some("") | Some("/"));
    match (uri.scheme_str(), uri.host()) {
        (Some("http"), Some("localhost" | "127.0.0.1")) => bare,
        (Some("https"), Some(host)) => {
            bare && uri.port().is_none()
                && DISCOVERY_ORIGIN_SUFFIXES.iter().any(|suffix| {
                    host.strip_suffix(suffix)
                        .is_some_and(|label| !label.is_empty() && label.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-'))
                })
        }
        _ => false,
    }
}

/// Discovery for other local tools, chiefly tinkerglass's PC Display panel: tells an allowed site
/// that eyepiece-pc runs, its version and where its web UI is. Carries nothing else (no token,
/// no settings), and only allowed origins get the CORS header that lets them read it.
async fn hello(State(app): State<App>, method: axum::http::Method, headers: HeaderMap) -> HttpResponse {
    let mut response = if method == axum::http::Method::OPTIONS {
        StatusCode::NO_CONTENT.into_response()
    } else {
        Json(serde_json::json!({
            "app": "eyepiece-pc",
            "version": env!("CARGO_PKG_VERSION"),
            "url": app.url.as_ref(),
        }))
        .into_response()
    };
    let headers_out = response.headers_mut();
    headers_out.insert(header::CACHE_CONTROL, header::HeaderValue::from_static("no-store"));
    headers_out.insert(header::VARY, header::HeaderValue::from_static("Origin"));
    if let Some(origin) = headers.get(header::ORIGIN).and_then(|o| o.to_str().ok()) {
        if discovery_origin_allowed(origin, &app.discovery_origins) {
            if let Ok(value) = header::HeaderValue::from_str(origin) {
                headers_out.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, value);
                // Private Network Access preflights (older Chrome) from a public site to loopback
                headers_out.insert("access-control-allow-private-network", header::HeaderValue::from_static("true"));
            }
        }
    }
    response
}

/// A request from eyepiece-pc's own page: the session token in `x-eyepiece-token` (a header other
/// sites cannot send without a preflight we never answer) and no cross-site origin.
fn own_page(app: &App, headers: &HeaderMap) -> bool {
    let token_ok = headers
        .get("x-eyepiece-token")
        .is_some_and(|token| token.len() == app.token.len() && bool::from(token.as_bytes().ct_eq(app.token.as_bytes())));
    let site_ok = headers.get("sec-fetch-site").is_none_or(|value| value == "same-origin")
        && (!headers.contains_key(header::ORIGIN) || allowed_header(headers, header::ORIGIN, &app.origins));
    token_ok && site_ok
}

/// The web UI's Quit: stops the stream and the server, like Ctrl-C or the tray icon's Quit.
async fn quit(State(app): State<App>, headers: HeaderMap) -> HttpResponse {
    if !own_page(&app, &headers) {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }
    app.shutdown.notify_one();
    ([(header::CACHE_CONTROL, "no-store")], Json(serde_json::json!({ "quitting": true }))).into_response()
}

/// The web UI's "Browse" for the video file: the page and eyepiece-pc run on the same computer
/// (loopback only), so eyepiece-pc shows the system's file dialog and returns the chosen path,
/// which the page then sets as the `input` setting. A browser file input cannot do this: it never
/// reveals the path, and uploading would copy a whole video. Needs the session token in
/// `x-eyepiece-token` (a header other sites cannot send without a preflight we never answer).
async fn pick_file(State(app): State<App>, headers: HeaderMap) -> HttpResponse {
    if !own_page(&app, &headers) {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }
    if app.picking.swap(true, Ordering::AcqRel) {
        return (StatusCode::CONFLICT, Json(serde_json::json!({ "error": "a file dialog is already open" }))).into_response();
    }
    let result = tokio::task::spawn_blocking(crate::desktop::pick_video_file).await;
    app.picking.store(false, Ordering::Release);
    let body = match result {
        Ok(Ok(path)) => serde_json::json!({ "path": path }),
        Ok(Err(error)) => serde_json::json!({ "error": error }),
        Err(_) => serde_json::json!({ "error": "file dialog failed" }),
    };
    ([(header::CACHE_CONTROL, "no-store")], Json(body)).into_response()
}

async fn upgrade(
    State(app): State<App>,
    headers: HeaderMap,
    Query(credentials): Query<Credentials>,
    ws: WebSocketUpgrade,
) -> HttpResponse {
    if !allowed_header(&headers, header::ORIGIN, &app.origins)
        || credentials.token.len() != app.token.len()
        || !bool::from(credentials.token.as_bytes().ct_eq(app.token.as_bytes()))
    {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }
    let id = app.next_client.fetch_add(1, Ordering::Relaxed);
    ws.max_message_size(MAX_FRAME + 4)
        .max_frame_size(MAX_FRAME + 4)
        .on_upgrade(move |socket| handle_client(socket, app, id))
}

fn decode_frame(frame: &[u8]) -> Result<Request, String> {
    if frame.len() < 5 || frame.len() > MAX_FRAME + 4 {
        return Err("invalid RPC frame length".into());
    }
    let length = u32::from_be_bytes(frame[..4].try_into().unwrap()) as usize;
    if length != frame.len() - 4 {
        return Err("invalid RPC frame length".into());
    }
    let request = decode_request(&frame[4..])?;
    if request.id.is_empty() || request.id.len() > 256 {
        return Err("RPC request id must contain 1 to 256 bytes".into());
    }
    Ok(request)
}

fn encode_frame(response: Response) -> Result<Vec<u8>, String> {
    eyepiece_link::proto::encode_frame(&response_envelope(response)?, MAX_FRAME)
        .map_err(|e| format!("encode RPC response: {e}"))
}

#[derive(Default)]
struct PreviewChanges {
    committed: bool,
    pending: VecDeque<(String, bool, Option<bool>)>,
}

impl PreviewChanges {
    fn begin(&mut self, id: &str, enabled: bool) -> Result<(), &'static str> {
        if self.pending.iter().any(|(pending, _, _)| pending == id) {
            return Err("duplicate pending preview request ID");
        }
        self.pending.push_back((id.to_string(), enabled, None));
        Ok(())
    }

    fn resolve(&mut self, id: &str, success: bool) -> Option<bool> {
        let (_, _, result) = self.pending.iter_mut().find(|(pending, _, _)| pending == id)?;
        *result = Some(success);
        while self.pending.front().is_some_and(|(_, _, result)| result.is_some()) {
            let (_, enabled, result) = self.pending.pop_front().unwrap();
            if result == Some(true) {
                self.committed = enabled;
            }
        }
        Some(self.pending.iter().rev()
            .find(|(_, _, result)| *result != Some(false))
            .map_or(self.committed, |(_, enabled, _)| *enabled))
    }
}

async fn handle_client(socket: WebSocket, app: App, client: u64) {
    let mut events = app.events.subscribe();
    let mut previews = app.previews.subscribe();
    let (mut output, mut input) = socket.split();
    let prefix = format!("{client}:");
    let mut preview = false;
    let mut preview_wait_key = true;
    let mut preview_changes = PreviewChanges::default();
    loop {
        let mut event = tokio::select! {
            message = input.next() => {
                match message {
                    Some(Ok(Message::Binary(data))) => {
                        let mut request = match decode_frame(&data) {
                            Ok(request) => request,
                            Err(error) => {
                                eprintln!("[eyepiece-pc] WebSocket request: {error}");
                                break;
                            }
                        };
                        if let Some(request::Command::SetPreview(value)) = &request.command {
                            if let Err(error) = preview_changes.begin(&request.id, value.enabled) {
                                eprintln!("[eyepiece-pc] WebSocket request: {error}");
                                break;
                            }
                            preview = value.enabled;
                            if preview {
                                previews = app.previews.subscribe();
                                preview_wait_key = true;
                            }
                        }
                        request.id = format!("{prefix}{}", request.id);
                        if app.requests.try_send(ClientInput::Request(client, request)).is_err() {
                            eprintln!("[eyepiece-pc] WebSocket request queue full or closed");
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(_))) | Some(Ok(Message::Pong(_))) => {}
                    Some(Ok(_)) => {
                        eprintln!("[eyepiece-pc] WebSocket requires binary RPC frames");
                        break;
                    }
                    Some(Err(error)) => {
                        eprintln!("[eyepiece-pc] WebSocket receive: {error}");
                        break;
                    }
                }
                continue;
            }
            event = events.recv() => {
                match event {
                    Ok(event) => event,
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        eprintln!("[eyepiece-pc] WebSocket client too slow");
                        break;
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            event = previews.recv(), if preview => {
                match event {
                    Ok(mut event) => {
                        if let Some(response::Event::Preview(frame)) = &mut event.event {
                            if preview_wait_key && !frame.key {
                                continue;
                            }
                            if preview_wait_key {
                                frame.discontinuity = true;
                                preview_wait_key = false;
                            }
                        }
                        event
                    }
                    Err(broadcast::error::RecvError::Lagged(count)) => {
                        eprintln!("[eyepiece-pc] WebSocket preview skipped {count} frames");
                        preview_wait_key = true;
                        if app.requests.try_send(ClientInput::Request(client, Request {
                            id: String::new(),
                            command: Some(request::Command::ForceIdr(wire::ForceIdr {})),
                        })).is_err() {
                            eprintln!("[eyepiece-pc] cannot request preview keyframe");
                        }
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        };
        if let Some(id) = event.id.strip_prefix(&prefix) {
            event.id = id.to_string();
            if let Some(enabled) = preview_changes.resolve(
                &event.id, !matches!(event.event, Some(response::Event::Error(_))),
            ) {
                preview = enabled;
            }
        } else if !event.id.is_empty() {
            continue;
        }
        let frame = match encode_frame(event) {
            Ok(frame) => frame,
            Err(error) => {
                eprintln!("[eyepiece-pc] WebSocket response: {error}");
                break;
            }
        };
        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            output.send(Message::Binary(frame.into())),
        )
        .await
        {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                eprintln!("[eyepiece-pc] WebSocket send: {error}");
                break;
            }
            Err(_) => {
                eprintln!("[eyepiece-pc] WebSocket send timed out");
                break;
            }
        }
    }
    if app
        .requests
        .send(ClientInput::Disconnected(client))
        .await
        .is_err()
    {
        eprintln!("[eyepiece-pc] RPC input closed while disconnecting client");
    }
}

fn common_codecs(clients: &HashMap<u64, Vec<String>>) -> Result<Vec<String>, String> {
    let codecs: Vec<_> = ["h264", "hevc"]
        .into_iter()
        .filter(|codec| {
            clients
                .values()
                .all(|codecs| codecs.iter().any(|c| c == codec))
        })
        .map(str::to_owned)
        .collect();
    if !clients.is_empty() && codecs.is_empty() {
        return Err("preview subscribers have no common codec".into());
    }
    Ok(codecs)
}

fn forward_requests(
    mut requests: async_mpsc::Receiver<ClientInput>,
    input: mpsc::Sender<Input>,
    output: mpsc::SyncSender<Response>,
) {
    let mut preview_clients = HashMap::new();
    let mut keyframe_clients = HashSet::new();
    while let Some(message) = requests.blocking_recv() {
        let request = match message {
            ClientInput::Request(client, mut request) => {
                if let Some(request::Command::SetPreview(preview)) = &mut request.command {
                    let result = (|| {
                        let mut clients = preview_clients.clone();
                        if preview.enabled {
                            let supported = super::preview::supported(&preview.supported_codecs)?;
                            clients.insert(
                                client,
                                supported
                                    .into_iter()
                                    .map(|c| super::preview::codec_name(c).to_string())
                                    .collect(),
                            );
                        } else {
                            clients.remove(&client);
                        }
                        let codecs = common_codecs(&clients)?;
                        preview_clients = clients;
                        if preview.enabled && preview.keyframes_only {
                            keyframe_clients.insert(client);
                        } else {
                            keyframe_clients.remove(&client);
                        }
                        Ok::<_, String>(codecs)
                    })();
                    match result {
                        Ok(codecs) => preview.supported_codecs = codecs,
                        Err(message) => {
                            if output
                                .send(Response {
                                    id: request.id,
                                    event: Some(response::Event::Error(wire::Error { message })),
                                })
                                .is_err()
                            {
                                break;
                            }
                            continue;
                        }
                    }
                    preview.enabled = !preview_clients.is_empty();
                    preview.keyframes_only = !preview_clients.is_empty()
                        && preview_clients
                            .keys()
                            .all(|client| keyframe_clients.contains(client));
                }
                request
            }
            ClientInput::Disconnected(client) => {
                if preview_clients.remove(&client).is_none() {
                    continue;
                }
                keyframe_clients.remove(&client);
                Request {
                    id: String::new(),
                    command: Some(request::Command::SetPreview(wire::SetPreview {
                        enabled: !preview_clients.is_empty(),
                        supported_codecs: common_codecs(&preview_clients)
                            .expect("removing subscribers cannot remove a common codec"),
                        keyframes_only: !preview_clients.is_empty()
                            && preview_clients
                                .keys()
                                .all(|client| keyframe_clients.contains(client)),
                    })),
                }
            }
        };
        if input.send(Input::Request(request)).is_err() {
            break;
        }
    }
}

fn broadcast_event(
    events: &broadcast::Sender<Response>,
    previews: &broadcast::Sender<Response>,
    event: Response,
) {
    if matches!(event.event, Some(response::Event::Preview(_))) {
        let _ = previews.send(event);
        return;
    }
    if !event.id.is_empty() && matches!(event.event, Some(response::Event::State(_))) {
        let mut broadcast = event.clone();
        broadcast.id.clear();
        let _ = events.send(broadcast);
    }
    let _ = events.send(event);
}

pub(super) fn run(startup: Startup, options: WebOptions) -> Result<(), WebError> {
    let port = configured_port()?;
    let url = format!("http://127.0.0.1:{port}/");
    let mut random = [0u8; 32];
    getrandom::fill(&mut random).map_err(|e| format!("generate RPC token: {e}"))?;
    let token: String = random.iter().map(|byte| format!("{byte:02x}")).collect();
    let mut hosts = HashSet::from([format!("127.0.0.1:{port}"), format!("localhost:{port}")]);
    let mut origins = HashSet::from([
        format!("http://127.0.0.1:{port}"),
        format!("http://localhost:{port}"),
    ]);
    match std::env::var("EYEP_DEV_ORIGIN") {
        Ok(value) => {
            let (origin, host) = origin(&value)?;
            hosts.insert(host);
            origins.insert(origin);
        }
        Err(std::env::VarError::NotPresent) => {}
        Err(error) => return Err(format!("invalid EYEP_DEV_ORIGIN: {error}").into()),
    }
    let discovery_origins: HashSet<String> = std::env::var("EYEPIECE_DISCOVERY_ORIGINS")
        .unwrap_or_default()
        .split(',')
        .map(|origin| origin.trim().trim_end_matches('/').to_string())
        .filter(|origin| !origin.is_empty())
        .collect();
    let site = web_dir().map_err(WebError::NotBuilt)?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("create RPC runtime: {e}"))?;
    let listener = match runtime.block_on(tokio::net::TcpListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, port)))) {
        Ok(listener) => listener,
        // most likely eyepiece-pc runs already (a second double-click): show that one
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse && options.open_browser && !options.start => {
            eprintln!("[eyepiece-pc] port {port} is in use, eyepiece-pc probably runs already: opening {url}");
            crate::desktop::open_url(&url)?;
            return Ok(());
        }
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            return Err(format!(
                "port {port} is in use: another eyepiece-pc already serves {url} (stop it, use --no-web, or set EYEPIECE_PORT)"
            )
            .into());
        }
        Err(e) => return Err(format!("bind RPC server: {e}").into()),
    };
    if options.tray && !options.terminal {
        crate::desktop::hide_console();
    }
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::sync_channel::<Response>(32);
    let worker_input = input_tx.clone();
    let request_output = output_tx.clone();
    let (worker_finished, worker_done) = tokio::sync::oneshot::channel();
    let worker = thread::spawn(move || {
        let result = run_worker(worker_input, input_rx, output_tx, Some(startup));
        let _ = worker_finished.send(result.as_ref().err().cloned());
        result
    });
    let (requests, requests_rx) = async_mpsc::channel(REQUEST_QUEUE);
    let request_input = input_tx.clone();
    let request_thread =
        thread::spawn(move || forward_requests(requests_rx, request_input, request_output));
    let (events, _) = broadcast::channel(CLIENT_QUEUE);
    let (previews, _) = broadcast::channel(CLIENT_QUEUE);
    let event_tx = events.clone();
    let preview_tx = previews.clone();
    let terminal = options.terminal;
    let output_thread = thread::spawn(move || {
        let mut phase = String::new();
        for event in output_rx {
            if terminal {
                print_event(&event, &mut phase);
            }
            broadcast_event(&event_tx, &preview_tx, event);
        }
    });
    let shutdown = Arc::new(Notify::new());
    if terminal && std::io::IsTerminal::is_terminal(&std::io::stdin()) {
        let input = input_tx.clone();
        let shutdown = shutdown.clone();
        // detached: it blocks on stdin, and the process exits after the server stops anyway
        thread::spawn(move || read_keys(input, shutdown));
    }
    let tray_exit = Arc::new(AtomicBool::new(false));
    #[cfg(feature = "tray")]
    let tray = options.tray.then(|| (input_tx.clone(), events.subscribe(), shutdown.clone(), url.clone()));
    let app = App {
        token: token.into(),
        hosts: Arc::new(hosts),
        origins: Arc::new(origins),
        discovery_origins: Arc::new(discovery_origins),
        url: url.clone().into(),
        requests,
        events,
        previews,
        next_client: Arc::new(AtomicU64::new(1)),
        picking: Arc::new(AtomicBool::new(false)),
        shutdown: shutdown.clone(),
    };
    let router = Router::new()
        .route("/api/session", get(session))
        .route("/api/rpc", get(upgrade))
        .route("/api/hello", get(hello).options(hello))
        .route("/api/pick-file", get(pick_file))
        .route("/api/quit", get(quit))
        .fallback_service(ServeDir::new(site))
        .layer(middleware::from_fn_with_state(app.clone(), protect))
        .with_state(app);
    eprintln!("[eyepiece-pc] Web UI: {url}");
    if let Ok(path) = super::settings_file() {
        eprintln!("[eyepiece-pc] settings: {}", path.display());
    }
    if options.open_browser {
        if let Err(e) = crate::desktop::open_url(&url) {
            eprintln!("[eyepiece-pc] {e}");
        }
    }
    let exit = tray_exit.clone();
    let serve = move || -> Result<(), WebError> {
        let result = runtime.block_on(async {
            tokio::select! {
                result = axum::serve(listener, router).into_future() => {
                    result.map_err(|e| format!("RPC server: {e}"))
                }
                result = tokio::signal::ctrl_c() => {
                    result.map_err(|e| format!("RPC shutdown signal: {e}"))
                }
                () = shutdown.notified() => Ok(()),
                result = worker_done => {
                    Err(result.ok().flatten().unwrap_or_else(|| "RPC worker stopped unexpectedly".into()))
                }
            }
        });
        exit.store(true, Ordering::Relaxed);
        let _ = input_tx.send(Input::Closed);
        drop(input_tx);
        drop(runtime);
        request_thread
            .join()
            .map_err(|_| "RPC request thread panicked".to_string())?;
        worker.join().map_err(|_| "RPC worker panicked".to_string())??;
        output_thread
            .join()
            .map_err(|_| "RPC output thread panicked".to_string())?;
        Ok(result?)
    };
    // the icon runs on this thread (macOS: it must be the main thread), the server beside it
    #[cfg(feature = "tray")]
    if let Some((input, events, shutdown, url)) = tray {
        let server = thread::spawn(serve);
        run_tray(input, events, &shutdown, &url, &tray_exit);
        return server.join().map_err(|_| WebError::Other("RPC server thread panicked".into()))?;
    }
    serve()
}

/// The tray icon until Quit (which stops the server) or the server's end. Without an icon (no
/// tray host, an error) eyepiece-pc keeps running: the web UI has Quit too.
#[cfg(feature = "tray")]
fn run_tray(
    input: mpsc::Sender<Input>,
    mut events: broadcast::Receiver<Response>,
    shutdown: &Notify,
    url: &str,
    exit: &AtomicBool,
) {
    let send = |command| {
        let _ = input.send(Input::Request(Request { id: String::new(), command: Some(command) }));
    };
    send(request::Command::GetState(wire::GetState {}));
    let act = |action| match action {
        crate::tray::Action::Open => {
            if let Err(e) = crate::desktop::open_url(url) {
                crate::desktop::error_box(&e);
            }
        }
        crate::tray::Action::Start => send(request::Command::Start(wire::Start {})),
        crate::tray::Action::Stop => send(request::Command::Stop(wire::Stop {})),
        crate::tray::Action::Quit => shutdown.notify_one(),
    };
    let mut phase = || {
        let mut latest = None;
        loop {
            match events.try_recv() {
                Ok(Response { event: Some(response::Event::State(state)), .. }) => latest = Some(state.phase),
                Ok(_) | Err(broadcast::error::TryRecvError::Lagged(_)) => {}
                Err(_) => return latest,
            }
        }
    };
    if let Err(e) = crate::tray::run(&act, &mut phase, exit) {
        eprintln!("[eyepiece-pc] no tray icon ({e}); quit from the web UI");
    }
}

/// The terminal's view of a web UI run: log lines, errors and stream phase changes.
fn print_event(event: &Response, phase: &mut String) {
    match &event.event {
        Some(response::Event::Log(log)) => eprintln!("[eyepiece-pc] {}", log.line),
        Some(response::Event::Error(error)) => eprintln!("[eyepiece-pc] error: {}", error.message),
        Some(response::Event::State(state)) if state.phase != *phase => {
            if !phase.is_empty() || state.phase != "idle" {
                eprintln!("[eyepiece-pc] stream {}", state.phase);
            }
            phase.clone_from(&state.phase);
        }
        _ => {}
    }
}

/// r = recenter, i = keyframe, q = quit (each + Enter), as in the command-line mode.
fn read_keys(input: mpsc::Sender<Input>, shutdown: Arc<Notify>) {
    use std::io::BufRead;
    for line in std::io::stdin().lock().lines() {
        let Ok(line) = line else { break };
        let command = match line.trim() {
            "r" => request::Command::Recenter(wire::Recenter {}),
            "i" => request::Command::ForceIdr(wire::ForceIdr {}),
            "q" => {
                shutdown.notify_one();
                break;
            }
            "" => continue,
            other => {
                eprintln!("[eyepiece-pc] unknown key {other:?} (r/i/q)");
                continue;
            }
        };
        if input.send(Input::Request(Request { id: String::new(), command: Some(command) })).is_err() {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn preview_failures_restore_the_latest_accepted_state() {
        let mut changes = PreviewChanges::default();
        changes.begin("first", true).unwrap();
        changes.begin("second", true).unwrap();
        assert!(changes.begin("first", false).is_err());
        assert_eq!(changes.resolve("first", false), Some(true));
        assert_eq!(changes.resolve("second", false), Some(false));
        changes.begin("third", true).unwrap();
        changes.begin("fourth", false).unwrap();
        assert_eq!(changes.resolve("fourth", true), Some(false));
        assert_eq!(changes.resolve("third", true), Some(false));
        changes.begin("fifth", true).unwrap();
        assert_eq!(changes.resolve("fifth", true), Some(true));
        assert_eq!(changes.resolve("unknown", false), None);
    }

    #[test]
    fn preview_bursts_do_not_evict_statistics() {
        let (events, mut receiver) = broadcast::channel(2);
        let (previews, mut preview_receiver) = broadcast::channel(2);
        broadcast_event(
            &events,
            &previews,
            Response {
                id: String::new(),
                event: Some(response::Event::Metrics(wire::Metrics::default())),
            },
        );
        for _ in 0..10 {
            broadcast_event(
                &events,
                &previews,
                Response {
                    id: String::new(),
                    event: Some(response::Event::Preview(wire::Preview::default())),
                },
            );
        }
        assert!(matches!(
            receiver.try_recv().unwrap().event,
            Some(response::Event::Metrics(_))
        ));
        assert!(matches!(
            preview_receiver.try_recv(),
            Err(broadcast::error::TryRecvError::Lagged(_))
        ));
    }

    #[test]
    fn discovery_answers_only_tinkerglass_and_local_origins() {
        let extra = HashSet::from(["https://example.org".to_string()]);
        for allowed in [
            "https://tinkerglass-org.github.io",
            "https://pr-12.tinkerglass-prs.pages.dev",
            "http://localhost:5173",
            "http://127.0.0.1:4173",
            "https://example.org",
        ] {
            assert!(discovery_origin_allowed(allowed, &extra), "{allowed}");
        }
        for denied in [
            "https://evil.example",
            "https://tinkerglass-prs.pages.dev",
            "https://a.b.tinkerglass-prs.pages.dev",
            "http://pr-12.tinkerglass-prs.pages.dev",
            "https://pr-12.tinkerglass-prs.pages.dev:8443",
            "https://pr-12.tinkerglass-prs.pages.dev.evil.example",
            "http://localhost.evil.example",
            "null",
        ] {
            assert!(!discovery_origin_allowed(denied, &extra), "{denied}");
        }
    }

    #[test]
    fn validates_local_origins() {
        assert!(origin("http://127.0.0.1:5173").is_ok());
        assert!(origin("http://localhost:5173").is_ok());
        assert!(origin("https://localhost:5173").is_err());
        assert!(origin("http://localhost.example:5173").is_err());
        assert!(origin("http://localhost:5173/path").is_err());
        assert!(origin("http://localhost:5173?x=1").is_err());
    }

    #[test]
    fn rejects_missing_duplicate_and_untrusted_headers() {
        let allowed = HashSet::from(["http://localhost:5173".to_string()]);
        let mut headers = HeaderMap::new();
        assert!(!allowed_header(&headers, header::ORIGIN, &allowed));
        headers.insert(header::ORIGIN, "http://localhost:5173".parse().unwrap());
        assert!(allowed_header(&headers, header::ORIGIN, &allowed));
        headers.append(header::ORIGIN, "http://localhost:5173".parse().unwrap());
        assert!(!allowed_header(&headers, header::ORIGIN, &allowed));
        headers.clear();
        headers.insert(header::ORIGIN, "http://example.com".parse().unwrap());
        assert!(!allowed_header(&headers, header::ORIGIN, &allowed));
    }

    #[test]
    fn validates_websocket_frames() {
        let request = Request {
            id: "request".into(),
            command: Some(request::Command::GetState(wire::GetState {})),
        };
        let bytes = super::super::request_envelope(request).encode_to_vec();
        let mut frame = (bytes.len() as u32).to_be_bytes().to_vec();
        frame.extend(bytes);
        assert_eq!(decode_frame(&frame).unwrap().id, "request");
        frame[3] += 1;
        assert!(decode_frame(&frame).is_err());
        assert!(decode_frame(&[0, 0, 0, 0]).is_err());
    }

    #[test]
    fn preview_is_enabled_until_last_subscriber_disconnects() {
        let (sender, receiver) = async_mpsc::channel(8);
        let (input, received) = mpsc::channel();
        let (output, _) = mpsc::sync_channel(8);
        let thread = thread::spawn(move || forward_requests(receiver, input, output));
        for (client, enabled) in [(1, true), (2, true), (1, false)] {
            sender
                .blocking_send(ClientInput::Request(
                    client,
                    Request {
                        id: client.to_string(),
                        command: Some(request::Command::SetPreview(wire::SetPreview {
                            enabled,
                            supported_codecs: vec!["h264".into()],
                            keyframes_only: false,
                        })),
                    },
                ))
                .unwrap();
        }
        sender.blocking_send(ClientInput::Disconnected(2)).unwrap();
        drop(sender);
        thread.join().unwrap();
        let states: Vec<_> = received
            .into_iter()
            .map(|message| match message {
                Input::Request(Request {
                    command: Some(request::Command::SetPreview(p)),
                    ..
                }) => p.enabled,
                _ => panic!("unexpected message"),
            })
            .collect();
        assert_eq!(states, vec![true, true, true, false]);
    }

    #[test]
    fn keyframe_only_preview_requires_every_client_to_opt_in() {
        let (sender, receiver) = async_mpsc::channel(8);
        let (input, received) = mpsc::channel();
        let (output, _) = mpsc::sync_channel(8);
        let thread = thread::spawn(move || forward_requests(receiver, input, output));
        for (client, keyframes_only) in [(1, false), (2, true), (1, true)] {
            sender
                .blocking_send(ClientInput::Request(
                    client,
                    Request {
                        id: client.to_string(),
                        command: Some(request::Command::SetPreview(wire::SetPreview {
                            enabled: true,
                            supported_codecs: vec!["h264".into()],
                            keyframes_only,
                        })),
                    },
                ))
                .unwrap();
        }
        drop(sender);
        thread.join().unwrap();
        let values: Vec<_> = received
            .into_iter()
            .map(|message| match message {
                Input::Request(Request {
                    command: Some(request::Command::SetPreview(preview)),
                    ..
                }) => preview.keyframes_only,
                _ => panic!("unexpected message"),
            })
            .collect();
        assert_eq!(values, vec![false, false, true]);
    }

    #[test]
    fn incompatible_preview_subscription_preserves_existing_clients() {
        let (sender, receiver) = async_mpsc::channel(8);
        let (input, received) = mpsc::channel();
        let (output, errors) = mpsc::sync_channel(8);
        let thread = thread::spawn(move || forward_requests(receiver, input, output));
        for (client, codecs) in [
            (1, vec!["h264", "hevc"]),
            (2, vec!["h264"]),
            (3, vec!["hevc"]),
        ] {
            sender
                .blocking_send(ClientInput::Request(
                    client,
                    Request {
                        id: client.to_string(),
                        command: Some(request::Command::SetPreview(wire::SetPreview {
                            enabled: true,
                            supported_codecs: codecs.into_iter().map(str::to_owned).collect(),
                            keyframes_only: false,
                        })),
                    },
                ))
                .unwrap();
        }
        sender.blocking_send(ClientInput::Disconnected(2)).unwrap();
        sender.blocking_send(ClientInput::Disconnected(1)).unwrap();
        drop(sender);
        thread.join().unwrap();
        let error = errors.recv().unwrap();
        assert_eq!(error.id, "3");
        assert!(matches!(error.event, Some(response::Event::Error(_))));
        let states: Vec<_> = received
            .into_iter()
            .map(|input| match input {
                Input::Request(Request {
                    command: Some(request::Command::SetPreview(p)),
                    ..
                }) => (p.enabled, p.supported_codecs),
                _ => panic!("unexpected input"),
            })
            .collect();
        assert_eq!(
            states,
            vec![
                (true, vec!["h264".into(), "hevc".into()]),
                (true, vec!["h264".into()]),
                (true, vec!["h264".into(), "hevc".into()]),
                (false, vec!["h264".into(), "hevc".into()]),
            ]
        );
    }
}
