//! Drawing the mouse pointer into a captured BGRX image, for capture APIs that hand the pointer
//! over separately (DXGI Desktop Duplication: the frame has no pointer unless the system drew a
//! software cursor into it). Platform-independent so it is tested everywhere.

/// When to draw the pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorMode {
    /// Only when the captured image does not already show it.
    Auto,
    On,
    Off,
}

impl std::str::FromStr for CursorMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "auto" => Ok(CursorMode::Auto),
            "on" => Ok(CursorMode::On),
            "off" => Ok(CursorMode::Off),
            _ => Err(format!("cursor mode {s}: expected auto, on or off")),
        }
    }
}

/// The three pointer shape formats of DXGI (and of Windows cursors in general).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeKind {
    /// 1 bpp: the AND mask on top of the XOR mask (`height` is twice the pointer height).
    Monochrome,
    /// 32 bpp BGRA, alpha blended.
    Color,
    /// 32 bpp BGR plus a mask byte: 0 = replace with the colour, 0xff = XOR the colour into the screen.
    MaskedColor,
}

#[derive(Clone, Debug)]
pub struct PointerShape {
    pub kind: ShapeKind,
    pub width: u32,
    /// As DXGI reports it (for monochrome: both masks).
    pub height: u32,
    pub pitch: u32,
    pub data: Vec<u8>,
}

/// What a shape pixel does to the screen pixel below it.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Keep,
    /// BGR plus alpha 0..255
    Blend([u8; 3], u8),
    Xor([u8; 3]),
}

impl PointerShape {
    /// The pointer's height on screen.
    pub fn rows(&self) -> u32 {
        match self.kind {
            ShapeKind::Monochrome => self.height / 2,
            _ => self.height,
        }
    }

    fn op(&self, x: u32, y: u32) -> Op {
        let pitch = self.pitch as usize;
        match self.kind {
            ShapeKind::Monochrome => {
                let bit = |row: u32| -> bool {
                    let byte = self.data.get(row as usize * pitch + x as usize / 8).copied().unwrap_or(0xff);
                    byte & (0x80 >> (x % 8)) != 0
                };
                let (and, xor) = (bit(y), bit(y + self.rows()));
                match (and, xor) {
                    (true, false) => Op::Keep,
                    (false, false) => Op::Blend([0, 0, 0], 255),
                    (false, true) => Op::Blend([255, 255, 255], 255),
                    (true, true) => Op::Xor([255, 255, 255]),
                }
            }
            ShapeKind::Color | ShapeKind::MaskedColor => {
                let i = y as usize * pitch + x as usize * 4;
                let Some(p) = self.data.get(i..i + 4) else { return Op::Keep };
                let bgr = [p[0], p[1], p[2]];
                if self.kind == ShapeKind::Color {
                    if p[3] == 0 { Op::Keep } else { Op::Blend(bgr, p[3]) }
                } else if p[3] == 0 {
                    Op::Blend(bgr, 255)
                } else if bgr == [0, 0, 0] {
                    Op::Keep
                } else {
                    Op::Xor(bgr)
                }
            }
        }
    }
}

/// A BGRX image to draw into.
pub struct Image<'a> {
    pub data: &'a mut [u8],
    pub width: u32,
    pub height: u32,
    pub stride: usize,
}

impl Image<'_> {
    fn pixel(&mut self, x: i64, y: i64) -> Option<&mut [u8]> {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return None;
        }
        let i = y as usize * self.stride + x as usize * 4;
        self.data.get_mut(i..i + 3)
    }
}

/// Draw `shape` with its top-left corner at (`left`, `top`) in image coordinates; parts outside
/// the image (a pointer at the edge, or on the neighbouring monitor) are clipped.
pub fn draw(image: &mut Image, shape: &PointerShape, left: i32, top: i32) {
    for y in 0..shape.rows() {
        for x in 0..shape.width {
            let op = shape.op(x, y);
            if op == Op::Keep {
                continue;
            }
            let Some(px) = image.pixel(left as i64 + x as i64, top as i64 + y as i64) else { continue };
            match op {
                Op::Keep => {}
                Op::Blend(bgr, a) => {
                    for c in 0..3 {
                        px[c] = ((bgr[c] as u32 * a as u32 + px[c] as u32 * (255 - a as u32) + 127) / 255) as u8;
                    }
                }
                Op::Xor(bgr) => {
                    for c in 0..3 {
                        px[c] ^= bgr[c];
                    }
                }
            }
        }
    }
}

/// Whether the image already shows `shape` at (`left`, `top`): a system that draws a software
/// cursor puts it into the desktop image, and drawing a second one would double it. Compares the
/// opaque pointer pixels (inverting ones are skipped: their result depends on the background);
/// a pointer on a background of its own colours still differs in its outline.
pub fn already_shown(image: &mut Image, shape: &PointerShape, left: i32, top: i32) -> bool {
    let (mut opaque, mut matching) = (0u32, 0u32);
    for y in 0..shape.rows() {
        for x in 0..shape.width {
            let Op::Blend(bgr, a) = shape.op(x, y) else { continue };
            if a < 224 {
                continue;
            }
            let Some(px) = image.pixel(left as i64 + x as i64, top as i64 + y as i64) else { continue };
            opaque += 1;
            if (0..3).all(|c| px[c].abs_diff(bgr[c]) <= 24) {
                matching += 1;
            }
        }
    }
    opaque >= 8 && matching * 10 >= opaque * 9
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 4x4 color pointer: black outline, white inside, transparent elsewhere (2x2 core).
    fn arrow() -> PointerShape {
        let mut data = vec![0u8; 4 * 4 * 4];
        for y in 0..4 {
            for x in 0..4 {
                let inner = (1..3).contains(&x) && (1..3).contains(&y);
                let p = if inner { [255, 255, 255, 255] } else { [0, 0, 0, 255] };
                data[(y * 4 + x) * 4..][..4].copy_from_slice(&p);
            }
        }
        PointerShape { kind: ShapeKind::Color, width: 4, height: 4, pitch: 16, data }
    }

    fn canvas(value: u8) -> Vec<u8> {
        vec![value; 8 * 8 * 4]
    }

    #[test]
    fn color_pointer_is_drawn_clipped_and_then_recognised() {
        let mut data = canvas(128);
        let mut image = Image { data: &mut data, width: 8, height: 8, stride: 32 };
        assert!(!already_shown(&mut image, &arrow(), 2, 2));
        draw(&mut image, &arrow(), 2, 2);
        assert!(already_shown(&mut image, &arrow(), 2, 2));
        assert_eq!(&data[(2 * 8 + 2) * 4..][..3], &[0, 0, 0]);
        assert_eq!(&data[(3 * 8 + 3) * 4..][..3], &[255, 255, 255]);
        assert_eq!(&data[0..3], &[128, 128, 128]);
        // at the edge: only the visible part, no panic
        let mut data = canvas(128);
        let mut image = Image { data: &mut data, width: 8, height: 8, stride: 32 };
        draw(&mut image, &arrow(), -2, 6);
        assert_eq!(&data[(6 * 8) * 4..][..3], &[0, 0, 0]); // shape (2, 0): outline
        assert_eq!(&data[(7 * 8) * 4..][..3], &[255, 255, 255]); // shape (2, 1): inside
        draw(&mut Image { data: &mut data, width: 8, height: 8, stride: 32 }, &arrow(), 100, -100);
    }

    #[test]
    fn a_white_background_does_not_hide_the_outline() {
        let mut data = canvas(255);
        let mut image = Image { data: &mut data, width: 8, height: 8, stride: 32 };
        assert!(!already_shown(&mut image, &arrow(), 2, 2));
    }

    #[test]
    fn monochrome_masks_set_clear_keep_and_invert() {
        // 8 pixels wide, 1 row: AND row then XOR row
        let shape = PointerShape {
            kind: ShapeKind::Monochrome,
            width: 4,
            height: 2,
            pitch: 1,
            data: vec![0b1010_0000, 0b0110_0000],
        };
        let mut data = canvas(100);
        draw(&mut Image { data: &mut data, width: 8, height: 8, stride: 32 }, &shape, 0, 0);
        // x0: and 1 xor 0 keep; x1: and 0 xor 1 white; x2: and 1 xor 1 invert; x3: and 0 xor 0 black
        assert_eq!(&data[0..3], &[100, 100, 100]);
        assert_eq!(&data[4..7], &[255, 255, 255]);
        assert_eq!(&data[8..11], &[155, 155, 155]);
        assert_eq!(&data[12..15], &[0, 0, 0]);
    }

    #[test]
    fn masked_color_replaces_or_xors() {
        let shape = PointerShape {
            kind: ShapeKind::MaskedColor,
            width: 3,
            height: 1,
            pitch: 12,
            data: vec![10, 20, 30, 0, 0xff, 0, 0, 0xff, 0, 0, 0, 0xff],
        };
        let mut data = canvas(0x0f);
        draw(&mut Image { data: &mut data, width: 8, height: 8, stride: 32 }, &shape, 0, 0);
        assert_eq!(&data[0..3], &[10, 20, 30]);
        assert_eq!(&data[4..7], &[0xf0, 0x0f, 0x0f]);
        assert_eq!(&data[8..11], &[0x0f, 0x0f, 0x0f]);
    }

    #[test]
    fn modes_parse() {
        assert_eq!("auto".parse::<CursorMode>(), Ok(CursorMode::Auto));
        assert!("yes".parse::<CursorMode>().is_err());
    }
}
