use super::{Input, MAX_FRAME, Request, Response, decode_request, request, response, response_envelope, run_worker, wire};
use crate::cli::Args;
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
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    thread,
};
use subtle::ConstantTimeEq;
use tokio::sync::{broadcast, mpsc as async_mpsc};
use tower_http::services::ServeDir;

const CLIENT_QUEUE: usize = 16;
const REQUEST_QUEUE: usize = 16;

#[derive(Clone)]
struct App {
    token: Arc<str>,
    hosts: Arc<HashSet<String>>,
    origins: Arc<HashSet<String>>,
    requests: async_mpsc::Sender<ClientInput>,
    events: broadcast::Sender<Response>,
    previews: broadcast::Sender<Response>,
    next_client: Arc<AtomicU64>,
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

/// The static web app: EYEPIECE_WEB_DIR if set; otherwise `web/` next to the executable (release
/// packages), then `web/build` of the checkout this binary was built from (development builds).
fn web_dir() -> Result<PathBuf, String> {
    if let Some(dir) = std::env::var_os("EYEPIECE_WEB_DIR").map(PathBuf::from) {
        return if dir.is_dir() { Ok(dir) } else { Err(format!("EYEPIECE_WEB_DIR {} does not exist", dir.display())) };
    }
    let packaged = std::env::current_exe().ok().and_then(|exe| exe.parent().map(|dir| dir.join("web")));
    let checkout = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../web/build");
    packaged
        .into_iter()
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
    );
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

pub(super) fn run(
    args: Args,
    overrides: HashSet<String>,
) -> Result<(), String> {
    let port = configured_port()?;
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
        Err(error) => return Err(format!("invalid EYEP_DEV_ORIGIN: {error}")),
    }
    let site = web_dir()?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("create RPC runtime: {e}"))?;
    let listener = runtime
        .block_on(tokio::net::TcpListener::bind(SocketAddr::from((
            Ipv4Addr::LOCALHOST,
            port,
        ))))
        .map_err(|e| format!("bind RPC server: {e}"))?;
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::sync_channel::<Response>(32);
    let worker_input = input_tx.clone();
    let request_output = output_tx.clone();
    let (worker_finished, worker_done) = tokio::sync::oneshot::channel();
    let worker = thread::spawn(move || {
        let result = run_worker(worker_input, input_rx, output_tx, Some((args, overrides)));
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
    let output_thread = thread::spawn(move || {
        for event in output_rx {
            broadcast_event(&event_tx, &preview_tx, event);
        }
    });
    let app = App {
        token: token.into(),
        hosts: Arc::new(hosts),
        origins: Arc::new(origins),
        requests,
        events,
        previews,
        next_client: Arc::new(AtomicU64::new(1)),
    };
    let router = Router::new()
        .route("/api/session", get(session))
        .route("/api/rpc", get(upgrade))
        .fallback_service(ServeDir::new(site))
        .layer(middleware::from_fn_with_state(app.clone(), protect))
        .with_state(app);
    eprintln!("[eyepiece-pc] Web UI: http://127.0.0.1:{port}/");
    let result = runtime.block_on(async {
        tokio::select! {
            result = axum::serve(listener, router).into_future() => {
                result.map_err(|e| format!("RPC server: {e}"))
            }
            result = tokio::signal::ctrl_c() => {
                result.map_err(|e| format!("RPC shutdown signal: {e}"))
            }
            result = worker_done => {
                Err(result.ok().flatten().unwrap_or_else(|| "RPC worker stopped unexpectedly".into()))
            }
        }
    });
    let _ = input_tx.send(Input::Closed);
    drop(input_tx);
    drop(runtime);
    request_thread
        .join()
        .map_err(|_| "RPC request thread panicked")?;
    worker.join().map_err(|_| "RPC worker panicked")??;
    output_thread
        .join()
        .map_err(|_| "RPC output thread panicked")?;
    result
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
