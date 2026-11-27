use super::{MAX_FRAME, Response, response, wire};
use crate::session::AuTap;
use eyepiece_av::ffi::*;
use eyepiece_av::{Codec, ConvertConfig, Converter, Encoder, EncoderConfig, encoder};
use std::sync::{
    Arc, Mutex,
    mpsc::{self, SyncSender},
};
use std::thread::{self, JoinHandle};
use std::time::Instant;

#[derive(Default)]
pub(super) struct Subscription {
    generation: u64,
    codecs: Option<Vec<Codec>>,
    keyframes_only: bool,
}

pub(super) fn supported(codecs: &[String]) -> Result<Vec<Codec>, String> {
    if codecs.is_empty() {
        return Ok(vec![Codec::H264, Codec::Hevc]);
    }
    let codecs: Vec<_> = [Codec::H264, Codec::Hevc]
        .into_iter()
        .filter(|codec| codecs.iter().any(|name| name == codec_name(*codec)))
        .collect();
    if codecs.is_empty() {
        return Err("no supported preview codec (h264, hevc)".into());
    }
    Ok(codecs)
}

pub(super) fn codec_name(codec: Codec) -> &'static str {
    match codec {
        Codec::H264 => "h264",
        Codec::Hevc => "hevc",
    }
}

impl Subscription {
    pub(super) fn set(&mut self, request: &wire::SetPreview) -> Result<(), String> {
        let codecs = if request.enabled {
            Some(supported(&request.supported_codecs)?)
        } else {
            None
        };
        self.generation += 1;
        self.codecs = codecs;
        self.keyframes_only = request.enabled && request.keyframes_only;
        Ok(())
    }

    fn target(&self, native: Codec) -> Option<Codec> {
        self.codecs.as_ref().map(|codecs| {
            if codecs.contains(&native) {
                native
            } else {
                codecs[0]
            }
        })
    }
}

struct Sample {
    data: Vec<u8>,
    key: bool,
    codec: Codec,
    generation: u64,
    timestamp_us: u64,
    discontinuity: bool,
    packed_alpha: bool,
}

pub(super) fn spawn(
    native: Codec,
    cfg: EncoderConfig,
    subscription: Arc<Mutex<Subscription>>,
    output: SyncSender<Response>,
) -> Result<(AuTap, JoinHandle<()>), String> {
    let (tx, rx) = mpsc::sync_channel::<Sample>(8);
    let current = subscription.clone();
    let worker = thread::Builder::new()
        .name("ep-rpc-preview".into())
        .spawn(move || {
            let mut transcoder: Option<Transcoder> = None;
            let mut waiting = true;
            for sample in rx {
                if current.lock().unwrap().generation != sample.generation {
                    continue;
                }
                if sample.discontinuity {
                    transcoder = None;
                    waiting = true;
                }
                if waiting && !sample.key {
                    continue;
                }
                if waiting {
                    transcoder = None;
                }
                let result = (|| -> Result<Vec<(Vec<u8>, bool, u64)>, String> {
                    if sample.codec == native {
                        return Ok(vec![(sample.data, sample.key, sample.timestamp_us)]);
                    }
                    if transcoder.is_none() {
                        transcoder = Some(Transcoder::new(native, sample.codec, &cfg)?);
                    }
                    transcoder
                        .as_mut()
                        .unwrap()
                        .encode(&sample.data, sample.timestamp_us, waiting)
                })();
                let packets = match result {
                    Ok(packets) => packets,
                    Err(message) => {
                        let _ = output.try_send(Response {
                            id: String::new(),
                            event: Some(response::Event::Error(wire::Error {
                                message: format!("preview: {message}"),
                            })),
                        });
                        eprintln!("[eyepiece-pc] preview: {message}");
                        break;
                    }
                };
                if current.lock().unwrap().generation != sample.generation {
                    continue;
                }
                for (access_unit, key, timestamp_us) in packets {
                    if waiting && !key {
                        continue;
                    }
                    if access_unit.len() > MAX_FRAME - 1024 {
                        waiting = true;
                        continue;
                    }
                    let sent = output.try_send(Response {
                        id: String::new(),
                        event: Some(response::Event::Preview(wire::Preview {
                            access_unit,
                            key,
                            codec: codec_name(sample.codec).into(),
                            timestamp_us,
                            discontinuity: waiting,
                            packed_alpha: sample.packed_alpha,
                        })),
                    });
                    waiting = sent.is_err();
                }
            }
        })
        .map_err(|e| format!("start preview worker: {e}"))?;
    let begin = Instant::now();
    let state = Mutex::new((u64::MAX, true));
    let tap: AuTap = Arc::new(move |data, key, packed_alpha, discontinuity| {
        let subscription = subscription.lock().unwrap();
        let Some(codec) = subscription.target(native) else {
            return;
        };
        if subscription.keyframes_only && !key {
            return;
        }
        let mut state = state.lock().unwrap();
        if state.0 != subscription.generation {
            *state = (subscription.generation, true);
        }
        if discontinuity {
            state.1 = true;
        }
        if state.1 && !key {
            return;
        }
        if data.len() > MAX_FRAME - 1024 {
            state.1 = true;
            return;
        }
        let sample = Sample {
            data: data.to_vec(),
            key,
            codec,
            generation: subscription.generation,
            timestamp_us: begin.elapsed().as_micros().min(u64::MAX as u128) as u64,
            discontinuity: state.1,
            packed_alpha,
        };
        state.1 = tx.try_send(sample).is_err();
    });
    Ok((tap, worker))
}

struct Transcoder {
    decoder: *mut AVCodecContext,
    packet: *mut AVPacket,
    cfg: EncoderConfig,
    name: String,
    converter: Option<Converter>,
    encoder: Option<Encoder>,
    size: (i32, i32),
    last_pts: i64,
}

impl Drop for Transcoder {
    fn drop(&mut self) {
        unsafe {
            av_packet_free(&mut self.packet);
            avcodec_free_context(&mut self.decoder);
        }
    }
}

impl Transcoder {
    fn new(native: Codec, target: Codec, cfg: &EncoderConfig) -> Result<Self, String> {
        let mut cfg = cfg.clone();
        cfg.codec = target;
        cfg.encoder = "software".into();
        cfg.preset = Some("ultrafast".into());
        cfg.tune = Some("zerolatency".into());
        cfg.profile = Some(
            if target == Codec::H264 {
                "high"
            } else {
                "main"
            }
            .into(),
        );
        cfg.intra_refresh = false;
        cfg.opts = vec![("threads".into(), "2".into())];
        let name = encoder::resolve(&cfg.encoder, target, &cfg.vaapi_device)?;
        unsafe {
            let codec = avcodec_find_decoder(match native {
                Codec::H264 => AVCodecID::AV_CODEC_ID_H264,
                Codec::Hevc => AVCodecID::AV_CODEC_ID_HEVC,
            });
            if codec.is_null() {
                return Err(format!("no {} decoder", codec_name(native)));
            }

            let decoder = avcodec_alloc_context3(codec);
            if decoder.is_null() {
                return Err("allocate preview decoder".into());
            }
            let result = Self {
                decoder,
                packet: av_packet_alloc(),
                cfg,
                name,
                converter: None,
                encoder: None,
                size: (0, 0),
                last_pts: -1,
            };
            if result.packet.is_null() {
                return Err("allocate preview packet".into());
            }
            (*decoder).thread_count = 2;
            (*decoder).thread_type = FF_THREAD_SLICE as i32;
            (*decoder).flags |= AV_CODEC_FLAG_LOW_DELAY as i32;
            // Microseconds survive decoding and encoding, including delayed packets.
            (*decoder).pkt_timebase = AVRational {
                num: 1,
                den: 1_000_000,
            };
            check(
                avcodec_open2(decoder, codec, std::ptr::null_mut()),
                "open preview decoder",
            )?;
            Ok(result)
        }
    }

    fn encode(
        &mut self,
        data: &[u8],
        timestamp_us: u64,
        idr: bool,
    ) -> Result<Vec<(Vec<u8>, bool, u64)>, String> {
        unsafe {
            check(
                av_new_packet(self.packet, data.len() as i32),
                "allocate preview AU",
            )?;
            std::ptr::copy_nonoverlapping(data.as_ptr(), (*self.packet).data, data.len());
            (*self.packet).pts = timestamp_us as i64;
            (*self.packet).dts = timestamp_us as i64;
            let result = avcodec_send_packet(self.decoder, self.packet);
            av_packet_unref(self.packet);
            check(result, "decode preview AU")?;
        }
        let mut output = Vec::new();
        loop {
            let frame = Frame::alloc();
            let result = unsafe { avcodec_receive_frame(self.decoder, frame.0) };
            if result == EAGAIN_ || result == AVERROR_EOF_ {
                break;
            }
            check(result, "receive preview frame")?;
            let size = (frame.width(), frame.height());
            if self.size != size {
                self.size = size;
                self.converter = None;
                self.encoder = None;
            }
            if self.converter.is_none() {
                self.converter = Some(Converter::new(
                    ConvertConfig {
                        width: frame.width() as u32,
                        height: frame.height() as u32,
                        letterbox: false,
                        target: encoder::target_for(encoder::family(&self.name)),
                        vaapi_device: self.cfg.vaapi_device.clone(),
                        gpu_scale_cpu_input: false,
                        extra_vf: None,
                        scale_mode: "hq".into(),
                        packed_alpha: None,
                    },
                    self.cfg.fps,
                ));
            }
            let pts_us = unsafe { (*frame.0).pts };
            let Some(frame) = self.converter.as_mut().unwrap().convert(frame)? else {
                continue;
            };
            if self.encoder.is_none() {
                self.encoder = Some(unsafe {
                    Encoder::open(&self.cfg, &self.name, &frame, std::ptr::null_mut())
                }?);
            }
            let mut packets = Vec::new();
            let pts =
                ((pts_us as f64 * self.cfg.fps as f64 / 1e6).round() as i64).max(self.last_pts + 1);
            self.last_pts = pts;
            self.encoder
                .as_mut()
                .unwrap()
                .encode(frame, pts, idr, &mut packets)?;
            output.extend(packets.into_iter().map(|p| {
                (
                    p.data,
                    p.key,
                    (p.pts.max(0) as u64 * 1_000_000) / u64::from(self.cfg.fps),
                )
            }));
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn config(codec: Codec) -> EncoderConfig {
        EncoderConfig {
            codec,
            encoder: "software".into(),
            fps: 30,
            bitrate: 200_000,
            gop: 30,
            vbv_frames: 4.0,
            slices: 1,
            preset: Some("ultrafast".into()),
            tune: Some("zerolatency".into()),
            profile: None,
            intra_refresh: false,
            opts: vec![(
                "x265-params".into(),
                "pools=none:frame-threads=1:log-level=error:repeat-headers=1".into(),
            )],
            vaapi_device: String::new(),
        }
    }

    fn request(codecs: &[&str]) -> wire::SetPreview {
        wire::SetPreview {
            enabled: true,
            supported_codecs: codecs.iter().map(|c| c.to_string()).collect(),
            keyframes_only: false,
        }
    }

    fn receive(rx: &mpsc::Receiver<Response>) -> wire::Preview {
        match rx
            .recv_timeout(Duration::from_secs(10))
            .unwrap()
            .event
            .unwrap()
        {
            response::Event::Preview(preview) => preview,
            event => panic!("unexpected event: {event:?}"),
        }
    }

    #[test]
    fn negotiates_native_or_fallback_and_rejects_invalid_updates() {
        let mut subscription = Subscription::default();
        subscription.set(&request(&["h264", "hevc"])).unwrap();
        assert_eq!(subscription.target(Codec::Hevc), Some(Codec::Hevc));
        subscription.set(&request(&["h264"])).unwrap();
        assert_eq!(subscription.target(Codec::Hevc), Some(Codec::H264));
        let generation = subscription.generation;
        assert!(subscription.set(&request(&["vp9"])).is_err());
        assert_eq!(subscription.generation, generation);
        assert_eq!(subscription.target(Codec::Hevc), Some(Codec::H264));
        subscription.set(&request(&[])).unwrap();
        assert_eq!(subscription.target(Codec::Hevc), Some(Codec::Hevc));
    }

    #[test]
    fn passthrough_preserves_bytes_and_resyncs_after_resubscription() {
        let subscription = Arc::new(Mutex::new(Subscription::default()));
        subscription
            .lock()
            .unwrap()
            .set(&request(&["h264"]))
            .unwrap();
        let (tx, rx) = mpsc::sync_channel(8);
        let (tap, worker) = spawn(
            Codec::H264,
            config(Codec::H264),
            subscription.clone(),
            tx,
        )
        .unwrap();
        tap(&[1], false, false, false);
        tap(&[2], true, false, false);
        let sample = receive(&rx);
        assert_eq!(sample.access_unit, vec![2]);
        assert!(sample.discontinuity && sample.key);
        assert!(!sample.packed_alpha);
        tap(&[3], false, false, false);
        assert!(!receive(&rx).discontinuity);
        tap(&[6], true, true, true);
        let changed = receive(&rx);
        assert!(changed.discontinuity && changed.packed_alpha);
        subscription
            .lock()
            .unwrap()
            .set(&wire::SetPreview {
                enabled: false,
                supported_codecs: Vec::new(),
                keyframes_only: false,
            })
            .unwrap();
        subscription
            .lock()
            .unwrap()
            .set(&request(&["h264"]))
            .unwrap();
        tap(&[4], false, false, false);
        tap(&[5], true, false, false);
        let sample = receive(&rx);
        assert_eq!(sample.access_unit, vec![5]);
        assert!(sample.discontinuity);
        drop(tap);
        worker.join().unwrap();
    }

    #[test]
    fn keyframes_only_subscription_filters_delta_frames() {
        let subscription = Arc::new(Mutex::new(Subscription::default()));
        let mut request = request(&["h264"]);
        request.keyframes_only = true;
        subscription.lock().unwrap().set(&request).unwrap();
        let (tx, rx) = mpsc::sync_channel(8);
        let (tap, worker) =
            spawn(Codec::H264, config(Codec::H264), subscription, tx).unwrap();
        tap(&[1], false, false, false);
        tap(&[2], true, false, false);
        let sample = receive(&rx);
        assert_eq!(sample.access_unit, vec![2]);
        tap(&[3], false, false, false);
        assert!(matches!(rx.try_recv(), Err(mpsc::TryRecvError::Empty)));
        drop(tap);
        worker.join().unwrap();
    }

    #[test]
    fn hevc_preview_is_reencoded_to_decodable_h264() {
        let cfg = config(Codec::Hevc);
        let name = encoder::resolve("software", cfg.codec, "").unwrap();
        let frame = Frame::alloc();
        unsafe {
            (*frame.0).width = 64;
            (*frame.0).height = 64;
            (*frame.0).format = AVPixelFormat::AV_PIX_FMT_YUV420P as i32;
            check(av_frame_get_buffer(frame.0, 32), "allocate test frame").unwrap();
            for plane in 0..3 {
                let rows = if plane == 0 { 64 } else { 32 };
                std::ptr::write_bytes(
                    (*frame.0).data[plane],
                    if plane == 0 { 64 } else { 128 },
                    (*frame.0).linesize[plane] as usize * rows,
                );
            }
        }
        let mut encoder =
            unsafe { Encoder::open(&cfg, &name, &frame, std::ptr::null_mut()) }.unwrap();
        let mut packets = Vec::new();
        for pts in 0..3 {
            let copy = Frame(unsafe { av_frame_clone(frame.0) });
            assert!(!copy.0.is_null());
            encoder.encode(copy, pts, pts == 0, &mut packets).unwrap();
        }
        assert_eq!(packets.len(), 3);
        let subscription = Arc::new(Mutex::new(Subscription::default()));
        subscription
            .lock()
            .unwrap()
            .set(&request(&["h264"]))
            .unwrap();
        let (tx, rx) = mpsc::sync_channel(8);
        let (tap, worker) = spawn(Codec::Hevc, cfg.clone(), subscription, tx).unwrap();
        let decoder = Transcoder::new(Codec::H264, Codec::H264, &cfg).unwrap();
        let mut last_timestamp = None;
        for (index, packet) in packets.iter().enumerate() {
            tap(&packet.data, packet.key, true, false);
            let preview = receive(&rx);
            assert_eq!(preview.codec, "h264");
            assert!(preview.packed_alpha);
            assert_eq!(preview.key, index == 0);
            assert_eq!(preview.discontinuity, index == 0);
            assert_ne!(preview.access_unit, packet.data);
            if let Some(timestamp) = last_timestamp {
                assert!(preview.timestamp_us > timestamp);
            }
            last_timestamp = Some(preview.timestamp_us);
            let decoded = Frame::alloc();
            unsafe {
                check(
                    av_new_packet(decoder.packet, preview.access_unit.len() as i32),
                    "allocate test packet",
                )
                .unwrap();
                std::ptr::copy_nonoverlapping(
                    preview.access_unit.as_ptr(),
                    (*decoder.packet).data,
                    preview.access_unit.len(),
                );
                check(
                    avcodec_send_packet(decoder.decoder, decoder.packet),
                    "decode H.264 preview",
                )
                .unwrap();
                av_packet_unref(decoder.packet);
                check(
                    avcodec_receive_frame(decoder.decoder, decoded.0),
                    "receive H.264 preview",
                )
                .unwrap();
                assert_eq!((decoded.width(), decoded.height()), (64, 64));
                assert!((*(*decoded.0).data[0] as i32 - 64).abs() <= 3);
            }
        }
        drop(tap);
        worker.join().unwrap();
    }
}
