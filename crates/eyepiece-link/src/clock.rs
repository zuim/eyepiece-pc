//! The PC clock used for timestamps in bridge video and ping messages.
//!
//! Linux: CLOCK_MONOTONIC in µs — the same clock DRM vblank timestamps and PipeWire use,
//! so capture timestamps from those sources can be used directly.
//! Windows: QueryPerformanceCounter in µs (DXGI's LastPresentTime uses the same counter).

#[cfg(unix)]
pub fn mono_us() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
    ts.tv_sec as u64 * 1_000_000 + ts.tv_nsec as u64 / 1000
}

#[cfg(windows)]
mod win {
    extern "system" {
        fn QueryPerformanceCounter(c: *mut i64) -> i32;
        fn QueryPerformanceFrequency(f: *mut i64) -> i32;
    }
    pub fn qpc() -> (i64, i64) {
        let (mut c, mut f) = (0i64, 1i64);
        unsafe {
            QueryPerformanceCounter(&mut c);
            QueryPerformanceFrequency(&mut f);
        }
        (c, f)
    }
}

#[cfg(windows)]
pub fn mono_us() -> u64 {
    let (c, f) = win::qpc();
    qpc_to_us(c, f)
}

/// QPC ticks → µs (Windows capture timestamps).
pub fn qpc_to_us(ticks: i64, freq: i64) -> u64 {
    ((ticks as i128 * 1_000_000) / freq.max(1) as i128) as u64
}

/// Sleep until `t_us` on the [`mono_us`] clock (no-op if it already passed).
pub fn sleep_until_us(t_us: u64) {
    let now = mono_us();
    if t_us > now {
        std::thread::sleep(std::time::Duration::from_micros(t_us - now));
    }
}
