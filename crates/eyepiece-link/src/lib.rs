//! eyepiece PC link: protocol, bridge connection, clock sync, stats, adb.
//! No FFmpeg, no platform capture code: usable from any frontend.

pub mod adb;
pub mod clock;
pub mod link;
pub mod net;
pub mod proto;
pub mod stats;
pub mod usb;

pub use link::{Link, LinkOptions, LinkTransport, LogFn};
