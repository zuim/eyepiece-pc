//! FFmpeg side of eyepiece-pc: wrap captured buffers as AVFrames, convert (GPU where
//! possible), encode with low-latency settings, emit Annex-B access units.

pub mod annexb;
pub mod convert;
pub mod encoder;
pub mod ffi;
pub mod hw;
#[cfg(target_os = "linux")]
pub mod simkms;
pub mod source;

pub use convert::{ConvertConfig, Converter, PackedAlpha, Target};
pub use encoder::{Codec, Encoder, EncoderConfig, Packet};
pub use source::{CaptureSource, FileSource, Source, SourceFrame, TestSource};
