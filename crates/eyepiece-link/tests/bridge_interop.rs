use eyepiece_link::proto::{self, wire};
use wire::envelope::Message as Event;
use eyepiece_link::{Link, LinkOptions, LinkTransport};
use prost::Message;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Arc;
use std::time::Duration;

fn start() -> (Child, ChildStdin, ChildStdout) {
    let path = std::env::var_os("EYEPIECE_WIRE_INTEROP").expect("set EYEPIECE_WIRE_INTEROP to the compiled C harness");
    let mut child = Command::new(path).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
    let stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    (child, stdin, stdout)
}

fn receive_envelope(stdout: &mut ChildStdout) -> wire::Envelope {
    let mut prefix = [0; 4];
    stdout.read_exact(&mut prefix).unwrap();
    let mut bytes = vec![0; u32::from_be_bytes(prefix) as usize];
    stdout.read_exact(&mut bytes).unwrap();
    proto::decode_envelope(&bytes, proto::EP_MAX_PAYLOAD as usize).unwrap()
}

fn receive(stdout: &mut ChildStdout) -> Event {
    receive_envelope(stdout).message.unwrap()
}

fn send(stdin: &mut ChildStdin, message: Event) {
    stdin.write_all(&proto::bridge_frame(message).unwrap()).unwrap();
}

fn send_request(stdin: &mut ChildStdin, id: u32, message: Event) {
    let envelope = wire::Envelope {
        id: Some(id.to_string()), sender: Some("pc".into()), recipient: Some("bridge".into()), message: Some(message),
    };
    stdin.write_all(&proto::encode_frame(&envelope, proto::EP_MAX_PAYLOAD as usize).unwrap()).unwrap();
}

fn capture_link_video(access_unit: &[u8]) -> Vec<u8> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        let info = wire::Info { version: proto::EP_VERSION, framebuffer_width: 1920, framebuffer_height: 1080, ..Default::default() };
        socket.write_all(&proto::bridge_frame(Event::Info(info)).unwrap()).unwrap();
        loop {
            let mut prefix = [0; 4];
            socket.read_exact(&mut prefix).unwrap();
            let mut bytes = vec![0; u32::from_be_bytes(prefix) as usize];
            socket.read_exact(&mut bytes).unwrap();
            let envelope = wire::Envelope::decode(&bytes[..]).unwrap();
            match envelope.message {
                Some(Event::Configure(_)) => {
                    let ack = wire::Envelope {
                        id: envelope.id, sender: None, recipient: None,
                        message: Some(Event::ConfigureAck(wire::Acknowledgement { status: 0 })),
                    };
                    socket.write_all(&proto::encode_frame(&ack, proto::EP_MAX_PAYLOAD as usize).unwrap()).unwrap();
                }
                Some(Event::Video(_)) => {
                    let mut framed = prefix.to_vec();
                    framed.extend_from_slice(&bytes);
                    return framed;
                }
                _ => {}
            }
        }
    });
    let options = LinkOptions {
        addr: addr.to_string(), transport: LinkTransport::Tcp,
        connect_timeout: Duration::from_secs(2), opentrack: None,
        print_pose: false, head_tracking: false,
    };
    let link = Link::connect(&options, Arc::new(|_| {})).unwrap();
    let until = std::time::Instant::now() + Duration::from_secs(2);
    while link.info().is_none() && std::time::Instant::now() < until {
        std::thread::sleep(Duration::from_millis(5));
    }
    let config = proto::StreamConfig { codec: proto::EP_CODEC_HEVC, layout: proto::EP_LAYOUT_MONO, pane: None };
    link.configure(config, Duration::from_secs(2)).unwrap();
    link.send_video(987, proto::EP_CODEC_HEVC, true, access_unit).unwrap();
    let bytes = server.join().unwrap();
    link.close();
    bytes
}

#[test]
#[ignore = "requires the nanopb C interop harness"]
fn nanopb_and_prost_interoperate() {
    let (mut child, mut stdin, mut stdout) = start();
    let Event::Info(info) = receive(&mut stdout) else { panic!("expected info"); };
    assert_eq!((info.version, info.framebuffer_width, info.framebuffer_height, info.decoder, info.flags),
               (proto::EP_VERSION, 1920, 1080, 2, proto::EP_INFO_RECONFIG | proto::EP_INFO_PANE_ALPHA));
    let Event::Pose(pose) = receive(&mut stdout) else { panic!("expected pose"); };
    assert_eq!((pose.device_us, pose.yaw, pose.pitch, pose.roll, pose.active, pose.source),
               (123456, 1.25, -2.5, 3.75, true, 3));
    let Event::Log(log) = receive(&mut stdout) else { panic!("expected log"); };
    assert_eq!(log.line, "bridge test");

    let pane = wire::Pane { x: 100, y: 50, width: 320, height: 240, format: 1 };
    let mut config = wire::Configure { codec: 1, layout: 0, pane: Some(pane) };
    send_request(&mut stdin, 699, Event::Configure(config.clone()));
    let Event::Frame(frame) = receive(&mut stdout) else { panic!("expected frame"); };
    assert_eq!((frame.pts_us, frame.received_us, frame.shown_us, frame.decode_us, frame.convert_us, frame.queue_depth, frame.dropped),
               (1, 0, 1, 100, 50, 320, 240));
    let envelope = receive_envelope(&mut stdout);
    assert_eq!(envelope.id.as_deref(), Some("699"));
    assert!(matches!(envelope.message, Some(Event::ConfigureAck(wire::Acknowledgement { status: 0 }))));
    send_request(&mut stdin, 698, Event::Configure(wire::Configure { codec: 0, layout: 1, pane: None }));
    let Event::Frame(frame) = receive(&mut stdout) else { panic!("expected frame"); };
    assert_eq!((frame.pts_us, frame.received_us, frame.shown_us), (0, 1, 0));
    assert!(matches!(receive(&mut stdout), Event::ConfigureAck(wire::Acknowledgement { status: 0 })));

    for (pts_us, codec, key, access_unit) in [
        (0, 0, false, vec![7]),
        (123456789, 1, true, vec![1, 2, 3, 255]),
    ] {
        let checksum: u32 = access_unit.iter().map(|&b| u32::from(b)).sum();
        let length = access_unit.len() as u32;
        send(&mut stdin, Event::Video(wire::Video { pts_us, codec, key, access_unit }));
        let Event::Frame(frame) = receive(&mut stdout) else { panic!("expected frame"); };
        assert_eq!((frame.pts_us, frame.received_us, frame.shown_us, frame.decode_us, frame.convert_us),
                   (pts_us, u64::from(codec), u64::from(key), checksum, length));
    }
    // Access unit precedes known and unknown video fields and an envelope field.
    let video = [0x22, 3, 1, 2, 3, 0x08, 42, 0x10, 1, 0x18, 1, 0x28, 99];
    let mut reordered = vec![0x92, 0x03, video.len() as u8]; // Envelope.video
    reordered.extend_from_slice(&video);
    reordered.extend_from_slice(&[0x1a, 2, b'p', b'c']); // recipient
    stdin.write_all(&(reordered.len() as u32).to_be_bytes()).unwrap();
    stdin.write_all(&reordered).unwrap();
    let Event::Frame(frame) = receive(&mut stdout) else { panic!("expected frame"); };
    assert_eq!((frame.pts_us, frame.received_us, frame.shown_us, frame.decode_us, frame.convert_us),
               (42, 1, 1, 6, 3));
    // Consecutive instances of the same message merge under protobuf semantics.
    let merged = [0x92, 0x03, 3, 0x22, 1, 7, 0x92, 0x03, 2, 0x08, 42];
    stdin.write_all(&(merged.len() as u32).to_be_bytes()).unwrap();
    stdin.write_all(&merged).unwrap();
    let Event::Frame(frame) = receive(&mut stdout) else { panic!("expected frame"); };
    assert_eq!((frame.pts_us, frame.decode_us, frame.convert_us), (42, 7, 1));
    let mut max_video = wire::Video {
        pts_us: 987, codec: 1, key: true,
        access_unit: vec![0xa5; proto::EP_MAX_PAYLOAD as usize],
    };
    loop {
        let len = max_video.encoded_len();
        let packet_len = 2 + prost::encoding::encoded_len_varint(len as u64) + len;
        if packet_len <= proto::EP_MAX_PAYLOAD as usize {
            assert_eq!(packet_len, proto::EP_MAX_PAYLOAD as usize);
            break;
        }
        max_video.access_unit.pop();
    }
    let access_unit = max_video.access_unit;
    stdin.write_all(&capture_link_video(&access_unit)).unwrap();
    let Event::Frame(frame) = receive(&mut stdout) else { panic!("expected frame"); };
    assert_eq!((frame.pts_us, frame.received_us, frame.shown_us, frame.decode_us, frame.convert_us),
               (987, 1, 1, 0xa5 * access_unit.len() as u32, access_unit.len() as u32));

    // Stereo cannot be combined with a pane, and the pane must fit the panel.
    config.layout = 1;
    send_request(&mut stdin, 700, Event::Configure(config.clone()));
    receive(&mut stdout);
    let envelope = receive_envelope(&mut stdout);
    assert_eq!(envelope.id.as_deref(), Some("700"));
    assert!(matches!(envelope.message, Some(Event::ConfigureAck(wire::Acknowledgement { status: 1 }))));
    config.layout = 0;
    config.pane.as_mut().unwrap().x = 1900;
    send_request(&mut stdin, 700, Event::Configure(config));
    receive(&mut stdout);
    let Event::ConfigureAck(ack) = receive(&mut stdout) else { panic!("expected configure ack"); };
    assert_eq!(ack.status, 1);
    for (alpha, expected) in [(0, 0), (128, 128), (256, 1)] {
        send_request(&mut stdin, 19, Event::GuiAlpha(wire::GuiAlpha { alpha }));
        let envelope = receive_envelope(&mut stdout);
        assert_eq!(envelope.id.as_deref(), Some("19"));
        let Some(Event::GuiAlphaAck(ack)) = envelope.message else { panic!("expected GUI alpha ack"); };
        assert_eq!(ack.status, expected);
    }
    for id in [0, u32::MAX] {
        send_request(&mut stdin, id, Event::GuiAlpha(wire::GuiAlpha { alpha: 50 }));
        let envelope = receive_envelope(&mut stdout);
        assert_eq!(envelope.id.as_deref(), Some(id.to_string().as_str()));
        assert!(matches!(envelope.message, Some(Event::GuiAlphaAck(wire::Acknowledgement { status: 50 }))));
    }
    for (alpha, expected) in [(0, 0), (128, 128), (255, 255), (256, 1)] {
        send_request(&mut stdin, 29, Event::PaneAlpha(wire::PaneAlpha { alpha }));
        let envelope = receive_envelope(&mut stdout);
        assert_eq!(envelope.id.as_deref(), Some("29"));
        let Some(Event::PaneAlphaAck(ack)) = envelope.message else { panic!("expected pane alpha ack"); };
        assert_eq!(ack.status, expected);
    }
    send(&mut stdin, Event::Ping(wire::Ping { pc_us: 12345 }));
    let Event::Pong(pong) = receive(&mut stdout) else { panic!("expected pong"); };
    assert_eq!((pong.pc_us, pong.device_us), (12345, 987654));
    send(&mut stdin, Event::Control(wire::Control { operation: 3, enabled: true }));
    let Event::Frame(control) = receive(&mut stdout) else { panic!("expected frame"); };
    assert_eq!((control.pts_us, control.received_us), (3, 1));
    let mut packet = proto::bridge_frame(Event::Ping(wire::Ping { pc_us: 42 })).unwrap();
    packet.extend_from_slice(&[0x98, 0x06, 0x01]); // Unknown packet field 99.
    let length = (packet.len() as u32 - 4).to_be_bytes();
    packet[..4].copy_from_slice(&length);
    stdin.write_all(&packet).unwrap();
    let Event::Pong(pong) = receive(&mut stdout) else { panic!("expected pong"); };
    assert_eq!(pong.pc_us, 42);
    drop(stdin);
    assert!(child.wait().unwrap().success());
}

#[test]
#[ignore = "requires the nanopb C interop harness"]
fn malformed_protobuf_is_rejected_by_bridge() {
    let mut cases = vec![
        vec![0, 0, 0, 2, 0x08, 0x80],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 3, 0x0a, 1, b'x'],
        vec![0, 0, 0, 3, 0xc2, 0x0c, 0],
        proto::bridge_frame(Event::Video(wire::Video::default())).unwrap(),
        proto::bridge_frame(Event::Video(wire::Video { codec: 2, access_unit: vec![1], ..Default::default() })).unwrap(),
        // A later video message must not reuse bytes from an earlier oneof value.
        vec![0, 0, 0, 12, 0x92, 0x03, 3, 0x22, 1, 0xaa,
             0x9a, 0x03, 0, 0x92, 0x03, 0],
        vec![0, 0, 0, 12, 0x92, 0x03, 3, 0x22, 1, 0xaa,
             0xa0, 0x02, 1, 0x92, 0x03, 0],
        // Configure requires a correlation ID.
        proto::bridge_frame(Event::Configure(wire::Configure::default())).unwrap(),
    ];
    for id in [None, Some("01"), Some("-1"), Some("4294967296"), Some("12345678901")] {
        let envelope = wire::Envelope {
            id: id.map(str::to_owned), sender: None, recipient: None,
            message: Some(Event::GuiAlpha(wire::GuiAlpha { alpha: 50 })),
        };
        cases.push(proto::encode_frame(&envelope, proto::EP_MAX_PAYLOAD as usize).unwrap());
    }
    for bytes in cases {
        let (mut child, mut stdin, mut stdout) = start();
        receive(&mut stdout);
        receive(&mut stdout);
        receive(&mut stdout);
        stdin.write_all(&bytes).unwrap();
        drop(stdin);
        assert!(!child.wait().unwrap().success());
    }
}
