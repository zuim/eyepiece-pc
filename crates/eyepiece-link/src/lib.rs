//! eyepiece PC link: protocol, bridge connection, the goggles' control link, clock sync, stats.
//! No FFmpeg, no platform capture code: usable from any frontend.

pub mod clock;
pub mod control;
pub mod link;
pub mod proto;
pub mod stats;
pub mod usb;

pub use link::{Link, LinkOptions, LinkTransport, LogFn};
