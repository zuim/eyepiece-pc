//! Diagnostic: does DXGI Desktop Duplication on this machine put the mouse pointer into the
//! captured image (a software cursor), and does eyepiece-pc draw it? Move the mouse while it runs.
//!
//!     cargo run -p eyepiece-capture --example dxgi-cursor -- [auto|on|off] [seconds] [output]

#[cfg(windows)]
fn main() {
    use eyepiece_capture::Capture;
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "auto".into()).parse().expect("auto, on or off");
    let seconds: f64 = args.next().map(|s| s.parse().expect("seconds")).unwrap_or(5.0);
    let output: u32 = args.next().map(|s| s.parse().expect("output index")).unwrap_or(0);
    let mut capture = eyepiece_capture::dxgi::DxgiCapture::open(0, output, 100.0, mode).expect("open DXGI");
    println!("{}", capture.describe());
    let (mut frames, mut drawn, mut shown_by_system, mut off_output) = (0, 0, 0, 0);
    let start = std::time::Instant::now();
    let mut last = None;
    while start.elapsed().as_secs_f64() < seconds {
        let Some(frame) = capture.next_frame(frames == 0).expect("capture") else {
            continue;
        };
        last = Some(frame);
        frames += 1;
        match capture.pointer_drawn() {
            Some(true) => drawn += 1,
            Some(false) => shown_by_system += 1,
            None => off_output += 1,
        }
    }
    println!(
        "{frames} frames: pointer drawn by us in {drawn}, already in the image in {shown_by_system}, \
         not on this output or no shape yet in {off_output}"
    );
    // DXGI_CURSOR_CROP=crop.png: the 96x96 pixels around the pointer of the last frame, to see
    // whether it shows once (only that region of the screen is written)
    if let (Ok(path), Some(frame)) = (std::env::var("DXGI_CURSOR_CROP"), last) {
        #[allow(irrefutable_let_patterns)]
        let eyepiece_capture::FrameData::Cpu(image) = frame.data else { return };
        let (px, py) = capture.pointer_position();
        let (x0, y0) = ((px - 32).max(0) as usize, (py - 32).max(0) as usize);
        let mut rgb = Vec::new();
        for y in y0..(y0 + 96).min(image.height as usize) {
            for x in x0..(x0 + 96).min(image.width as usize) {
                let p = &image.data[y * image.stride + x * 4..][..3];
                rgb.extend_from_slice(&[p[2], p[1], p[0]]);
            }
        }
        let w = (x0 + 96).min(image.width as usize) - x0;
        std::fs::write(&path, png(w as u32, (rgb.len() / 3 / w) as u32, &rgb)).expect("write crop");
        println!("crop around ({px}, {py}) written to {path}");
    }
}

/// A minimal RGB PNG (stored deflate blocks), enough for a diagnostic crop.
#[cfg(windows)]
fn png(width: u32, height: u32, rgb: &[u8]) -> Vec<u8> {
    fn crc(data: &[u8]) -> u32 {
        let mut c = 0xffff_ffffu32;
        for &b in data {
            c ^= b as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 { 0xedb8_8320 ^ (c >> 1) } else { c >> 1 };
            }
        }
        !c
    }
    fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        let start = out.len();
        out.extend_from_slice(kind);
        out.extend_from_slice(data);
        let c = crc(&out[start..]);
        out.extend_from_slice(&c.to_be_bytes());
    }
    let mut raw = Vec::new();
    for row in rgb.chunks(width as usize * 3) {
        raw.push(0);
        raw.extend_from_slice(row);
    }
    let mut z = vec![0x78, 0x01];
    let blocks: Vec<&[u8]> = raw.chunks(65535).collect();
    for (i, block) in blocks.iter().enumerate() {
        z.push((i + 1 == blocks.len()) as u8);
        z.extend_from_slice(&(block.len() as u16).to_le_bytes());
        z.extend_from_slice(&(!(block.len() as u16)).to_le_bytes());
        z.extend_from_slice(block);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &byte in &raw {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    z.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut header = Vec::new();
    header.extend_from_slice(&width.to_be_bytes());
    header.extend_from_slice(&height.to_be_bytes());
    header.extend_from_slice(&[8, 2, 0, 0, 0]);
    chunk(&mut out, b"IHDR", &header);
    chunk(&mut out, b"IDAT", &z);
    chunk(&mut out, b"IEND", &[]);
    out
}

#[cfg(not(windows))]
fn main() {
    eprintln!("DXGI is Windows only");
}
