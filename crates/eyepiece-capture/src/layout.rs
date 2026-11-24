#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn fit_rect(
    source: (u32, u32),
    output: (u32, u32),
    letterbox: bool,
) -> Result<CaptureRect, String> {
    let (source_width, source_height) = source;
    let (output_width, output_height) = output;
    if source_width == 0
        || source_height == 0
        || output_width < 2
        || output_height < 2
        || output_width % 2 != 0
        || output_height % 2 != 0
    {
        return Err(
            "Capture needs a nonempty source and even output dimensions of at least 2 pixels"
                .into(),
        );
    }
    let (width, height) = if !letterbox {
        output
    } else if u64::from(source_width) * u64::from(output_height)
        > u64::from(output_width) * u64::from(source_height)
    {
        let height =
            (u64::from(source_height) * u64::from(output_width) / u64::from(source_width)) as u32;
        (output_width, (height / 2 * 2).max(2))
    } else {
        let width =
            (u64::from(source_width) * u64::from(output_height) / u64::from(source_height)) as u32;
        ((width / 2 * 2).max(2), output_height)
    };
    Ok(CaptureRect {
        x: (output_width - width) / 4 * 2,
        y: (output_height - height) / 4 * 2,
        width,
        height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_display_aspect_and_centers_black_bars() {
        assert_eq!(
            fit_rect((2560, 1600), (1920, 1080), true).unwrap(),
            CaptureRect {
                x: 96,
                y: 0,
                width: 1728,
                height: 1080
            }
        );
        assert_eq!(
            fit_rect((3840, 2160), (1920, 1080), true).unwrap(),
            CaptureRect {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080
            }
        );
    }

    #[test]
    fn portrait_rect_is_chroma_aligned_and_inside_output() {
        let rect = fit_rect((900, 1600), (1920, 1080), true).unwrap();
        assert_eq!((rect.width, rect.height), (606, 1080));
        assert_eq!((rect.x % 2, rect.y % 2), (0, 0));
        assert!(rect.x + rect.width <= 1920);
        assert!(rect.y + rect.height <= 1080);
    }

    #[test]
    fn stretch_fills_output_and_invalid_sizes_are_rejected() {
        assert_eq!(
            fit_rect((2560, 1600), (1920, 1080), false).unwrap(),
            CaptureRect {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080
            }
        );
        for (source, output) in [
            ((0, 1600), (1920, 1080)),
            ((900, 0), (1920, 1080)),
            ((900, 1600), (1921, 1080)),
            ((900, 1600), (1920, 1)),
        ] {
            assert!(fit_rect(source, output, true).is_err());
        }
    }
}
