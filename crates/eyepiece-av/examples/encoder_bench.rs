use eyepiece_av::{
    encoder, ffi, Codec, ConvertConfig, Converter, Encoder, EncoderConfig, Source, TestSource,
};
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let alias = args.first().map(String::as_str).unwrap_or("x264");
    let codec: Codec = args.get(1).map(String::as_str).unwrap_or("h264").parse()?;
    let fps = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("100")
        .parse::<u32>()
        .map_err(|error| error.to_string())?;
    let count = args
        .get(3)
        .map(String::as_str)
        .unwrap_or("600")
        .parse::<usize>()
        .map_err(|error| error.to_string())?;
    if fps == 0 || count == 0 {
        return Err("fps and frame count must be positive".into());
    }
    ffi::set_log_level(ffi::AV_LOG_WARNING as i32);
    let config = EncoderConfig {
        codec,
        encoder: alias.into(),
        fps,
        bitrate: 20_000_000,
        gop: 2 * fps,
        vbv_frames: 4.0,
        slices: 4,
        preset: std::env::var("BENCH_PRESET").ok(),
        tune: None,
        profile: None,
        intra_refresh: false,
        opts: vec![],
        vaapi_device: "/dev/dri/renderD128".into(),
    };
    let name = encoder::resolve(alias, codec, &config.vaapi_device)?;
    let mode = args.get(4).map(String::as_str).unwrap_or("test");
    let mut source: Box<dyn Source> = Box::new(TestSource::new(
        1920,
        1080,
        1_000_000,
        Some(&format!("testsrc2=size=1920x1080:rate={fps}")),
    )?);
    if mode != "test" {
        if mode != "native" && mode != "readback" && mode != "bgra" {
            return Err("input mode must be test, native, readback, or bgra".into());
        }
        if mode == "native" && encoder::family(&name) != encoder::Family::VideoToolbox {
            return Err("native input requires a VideoToolbox encoder".into());
        }
        #[cfg(target_os = "macos")]
        {
            let display_id = std::env::var("VT_DISPLAY_ID")
                .ok()
                .map(|value| value.parse::<u32>().map_err(|error| error.to_string()))
                .transpose()?;
            let capture = if mode == "bgra" {
                eyepiece_capture::screencapturekit::ScreenCapture::open(display_id, fps, false)?
            } else {
                eyepiece_capture::screencapturekit::ScreenCapture::open_native(
                    display_id,
                    fps,
                    false,
                    (1920, 1080),
                    true,
                )?
            };
            source = Box::new(eyepiece_av::CaptureSource::new(Box::new(capture), ""));
        }
        #[cfg(not(target_os = "macos"))]
        return Err("screen benchmark requires macOS".into());
    }
    let mut converter = Converter::new(
        ConvertConfig {
            width: 1920,
            height: 1080,
            letterbox: true,
            target: if mode == "readback" {
                encoder::target_for(encoder::Family::VideoToolbox)
            } else {
                encoder::target_for(encoder::family(&name))
            },
            vaapi_device: config.vaapi_device.clone(),
            gpu_scale_cpu_input: false,
            extra_vf: None,
            scale_mode: "hq".into(),
        },
        fps,
    );
    let warmup = 60;
    let mut encoder: Option<Encoder> = None;
    let mut open_time = Duration::ZERO;
    let mut encode_times = Vec::new();
    let mut packet_times = Vec::new();
    let mut submitted = Vec::new();
    let mut captured = Vec::new();
    let mut capture_packet_times = Vec::new();
    let mut conversion_times = Vec::new();
    let mut input_ages = Vec::new();
    let mut capture_times = Vec::new();
    let mut packet_sizes = Vec::new();
    let mut dump = std::env::var("BENCH_DUMP")
        .ok()
        .map(std::fs::File::create)
        .transpose()
        .map_err(|error| error.to_string())?;
    let mut packets = Vec::new();
    let mut output_frames = 0;
    let mut bytes = 0;
    let mut measured_start = Instant::now();
    let mut collect = |packets: &mut Vec<eyepiece_av::Packet>,
                       submitted: &[Instant],
                       captured: &[u64]| {
        let now = Instant::now();
        for packet in packets.drain(..) {
            if let Ok(index) = usize::try_from(packet.pts) {
                if index >= warmup && index < submitted.len() {
                    packet_times.push(now.duration_since(submitted[index]).as_secs_f64() * 1000.0);
                    capture_packet_times.push(
                        eyepiece_link::clock::mono_us().saturating_sub(captured[index]) as f64
                            / 1000.0,
                    );
                    output_frames += 1;
                    bytes += packet.data.len();
                    packet_sizes.push(packet.data.len() as f64);
                }
                if let Some(file) = &mut dump {
                    use std::io::Write;
                    file.write_all(&packet.data).expect("benchmark dump write");
                }
            }
        }
    };
    let run_start = Instant::now();
    for index in 0..warmup + count {
        let source_frame = loop {
            if let Some(frame) = source.next(false)? {
                break frame;
            }
            if run_start.elapsed().as_secs_f64() > (warmup + count) as f64 / fps as f64 * 4.0 + 5.0
            {
                return Err(
                    "capture did not supply enough fresh frames; animate the display".into(),
                );
            }
        };
        captured.push(source_frame.t_us);
        if index >= warmup {
            input_ages.push(
                eyepiece_link::clock::mono_us().saturating_sub(source_frame.t_us) as f64 / 1000.0,
            );
            capture_times.push(source_frame.t_us);
        }
        let frame = source_frame.frame;
        let convert_start = Instant::now();
        let frame = if mode == "native" {
            frame
        } else {
            converter.convert(frame)?.ok_or("missing converted frame")?
        };
        if index >= warmup {
            conversion_times.push(convert_start.elapsed().as_secs_f64() * 1000.0);
        }
        if encoder.is_none() {
            let start = Instant::now();
            encoder =
                Some(unsafe { Encoder::open(&config, &name, &frame, (*frame.0).hw_frames_ctx) }?);
            open_time = start.elapsed();
        }
        if index == warmup {
            measured_start = Instant::now();
        }
        let start = Instant::now();
        submitted.push(start);
        encoder
            .as_mut()
            .unwrap()
            .encode(frame, index as i64, index == 0, &mut packets)?;
        if index >= warmup {
            encode_times.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        collect(&mut packets, &submitted, &captured);
    }
    encoder.as_mut().unwrap().finish(&mut packets)?;
    collect(&mut packets, &submitted, &captured);
    let elapsed = measured_start.elapsed().as_secs_f64();
    let encode_total: f64 = encode_times.iter().sum();
    encode_times.sort_by(f64::total_cmp);
    packet_times.sort_by(f64::total_cmp);
    capture_packet_times.sort_by(f64::total_cmp);
    conversion_times.sort_by(f64::total_cmp);
    input_ages.sort_by(f64::total_cmp);
    packet_sizes.sort_by(f64::total_cmp);
    let percentile = |values: &[f64], percent: usize| {
        values
            .get((values.len().saturating_sub(1) * percent) / 100)
            .copied()
            .unwrap_or(f64::NAN)
    };
    println!("{}", encoder.as_ref().unwrap().description);
    println!("input={mode}; 60 warmup frames; generation/conversion excluded from encode timing.");
    println!("open {:.1} ms | packets {output_frames}/{count} | encode p50 {:.2} p95 {:.2} ms | packet p50 {:.2} p95 {:.2} ms | encode-call-budget {:.1} fps | pipeline {:.1} fps | stream {:.1} Mbit/s",
        open_time.as_secs_f64() * 1000.0, percentile(&encode_times, 50), percentile(&encode_times, 95),
        percentile(&packet_times, 50), percentile(&packet_times, 95), count as f64 * 1000.0 / encode_total,
        output_frames as f64 / elapsed, bytes as f64 * 8.0 * fps as f64 / count as f64 / 1e6);
    println!(
        "capture-callback-to-packet p50 {:.2} p95 {:.2} ms",
        percentile(&capture_packet_times, 50),
        percentile(&capture_packet_times, 95)
    );
    let quantiles = |values: &[f64]| {
        [
            percentile(values, 50),
            percentile(values, 95),
            percentile(values, 99),
        ]
    };
    let capture_fps = if capture_times.len() > 1 {
        (capture_times.len() - 1) as f64 * 1e6
            / capture_times
                .last()
                .unwrap()
                .saturating_sub(capture_times[0]) as f64
    } else {
        0.0
    };
    println!("RESULT {{\"frames\":{count},\"packets\":{output_frames},\"fps\":{},\"capture_fps\":{capture_fps},\"encode_ms\":{:?},\"conversion_ms\":{:?},\"input_age_ms\":{:?},\"packet_ms\":{:?},\"capture_ms\":{:?},\"frame_bytes\":{:?},\"mbps_nominal\":{}}}",
        output_frames as f64 / elapsed, quantiles(&encode_times), quantiles(&conversion_times),
        quantiles(&input_ages), quantiles(&packet_times), quantiles(&capture_packet_times), quantiles(&packet_sizes),
        bytes as f64 * 8.0 * fps as f64 / count as f64 / 1e6);
    if output_frames != count {
        return Err("encoder did not return every measured frame".into());
    }
    Ok(())
}
