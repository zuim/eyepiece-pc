//! Eyepiece PC streaming library: everything the `eyepiece-pc` CLI and browser UI
//! need. Build a [`session::Config`] (or a [`cli::Args`] and [`cli::prepare`] it), call
//! [`session::run`] on a thread, steer it with [`session::Control`], receive log lines,
//! [`eyepiece_link::stats::Snapshot`]s, the link and the encoded stream via [`session::Events`].

pub mod cli;
pub mod desktop;
pub mod goggles;
pub mod preview;
pub mod rpc;
pub mod session;
#[cfg(feature = "tray")]
mod tray;

pub use session::{run, Config, Control, Events, SourceKind};
pub use eyepiece_av as av;
pub use eyepiece_capture as capture;
pub use eyepiece_link as link;
