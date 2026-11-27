//! Per-interval statistics (same line as eyepiece_stream.py, plus PC-side encode/skip/IDR fields).

#[derive(Default, Clone, Debug)]
pub struct Interval {
    /// AUs sent / bytes sent (payload)
    pub sent: u32,
    pub bytes: u64,
    /// frames the capture backend delivered
    pub captured: u32,
    /// frames not encoded because the link was still busy (drop instead of queue)
    pub skipped: u32,
    /// IDRs forced (connect, bridge overflow, stall)
    pub idr: u32,
    /// goggles: EP_FRAME reports
    pub shown: u32,
    /// capture→shown µs (clock-synced), per shown frame
    pub lat: Vec<i64>,
    /// capture→AU fully received on the goggles µs (clock-synced), per shown frame
    pub recv: Vec<i64>,
    pub dec: Vec<u32>,
    pub cvt: Vec<u32>,
    pub q_max: u32,
    /// bridge's cumulative drop counter (last seen)
    pub dropped: u32,
    pub poses: u32,
    /// capture → AU handed to the socket, µs
    pub cap_send: Vec<u32>,
    /// capture → encoder input (convert/scale/upload), µs
    pub cap_enc: Vec<u32>,
    /// encoder in → AU out, µs
    pub enc: Vec<u32>,
}

impl Interval {
    pub fn reset_keep_totals(&mut self) -> Interval {
        let dropped = self.dropped;
        let out = std::mem::take(self);
        self.dropped = dropped;
        out
    }

    pub fn extend(&mut self, other: &Interval) {
        self.sent += other.sent;
        self.bytes += other.bytes;
        self.captured += other.captured;
        self.skipped += other.skipped;
        self.idr += other.idr;
        self.shown += other.shown;
        self.lat.extend_from_slice(&other.lat);
        self.recv.extend_from_slice(&other.recv);
        self.dec.extend_from_slice(&other.dec);
        self.cvt.extend_from_slice(&other.cvt);
        self.q_max = self.q_max.max(other.q_max);
        self.dropped = other.dropped;
        self.poses += other.poses;
        self.cap_send.extend_from_slice(&other.cap_send);
        self.cap_enc.extend_from_slice(&other.cap_enc);
        self.enc.extend_from_slice(&other.enc);
    }
}

pub fn pct_ms<T: Copy + Into<f64>>(v: &[T], q: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mut s: Vec<f64> = v.iter().map(|&x| x.into()).collect();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    s[((s.len() as f64 * q) as usize).min(s.len() - 1)] / 1000.0
}

fn pct_i64(v: &[i64], q: f64) -> f64 {
    let f: Vec<f64> = v.iter().map(|&x| x as f64).collect();
    pct_ms(&f, q)
}

/// Computed numbers for one interval (what a UI would display).
#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub interval_s: f64,
    pub report_interval_s: f64,
    pub sent_fps: f64,
    pub mbps: f64,
    pub shown_fps: f64,
    pub lat_p50_ms: f64,
    pub lat_p95_ms: f64,
    pub cap_send_p50_ms: f64,
    pub cap_send_p95_ms: f64,
    pub cap_enc_p50_ms: f64,
    pub enc_p50_ms: f64,
    pub dec_p50_ms: f64,
    pub cvt_p50_ms: f64,
    pub q_max: u32,
    pub dropped: u32,
    pub skipped: u32,
    pub idr: u32,
    pub captured_fps: f64,
    pub pose_hz: f64,
    pub rtt_ms: f64,
    pub yaw_pitch: Option<(f32, f32)>,
    // p95s and capture→received (not in `line()`; for UIs)
    pub cap_enc_p95_ms: f64,
    pub enc_p95_ms: f64,
    pub dec_p95_ms: f64,
    pub cvt_p95_ms: f64,
    pub recv_p50_ms: f64,
    pub recv_p95_ms: f64,
}

impl Snapshot {
    pub fn from_interval(s: &Interval, interval_s: f64, rtt_us: Option<u64>, pose: Option<(f32, f32)>) -> Snapshot {
        Snapshot {
            interval_s,
            report_interval_s: interval_s,
            sent_fps: s.sent as f64 / interval_s,
            mbps: s.bytes as f64 * 8.0 / interval_s / 1e6,
            shown_fps: s.shown as f64 / interval_s,
            lat_p50_ms: pct_i64(&s.lat, 0.5),
            lat_p95_ms: pct_i64(&s.lat, 0.95),
            cap_send_p50_ms: pct_ms(&s.cap_send, 0.5),
            cap_send_p95_ms: pct_ms(&s.cap_send, 0.95),
            cap_enc_p50_ms: pct_ms(&s.cap_enc, 0.5),
            enc_p50_ms: pct_ms(&s.enc, 0.5),
            dec_p50_ms: pct_ms(&s.dec, 0.5),
            cvt_p50_ms: pct_ms(&s.cvt, 0.5),
            q_max: s.q_max,
            dropped: s.dropped,
            skipped: s.skipped,
            idr: s.idr,
            captured_fps: s.captured as f64 / interval_s,
            pose_hz: s.poses as f64 / interval_s,
            rtt_ms: rtt_us.map(|r| r as f64 / 1000.0).unwrap_or(f64::NAN),
            yaw_pitch: pose,
            cap_enc_p95_ms: pct_ms(&s.cap_enc, 0.95),
            enc_p95_ms: pct_ms(&s.enc, 0.95),
            dec_p95_ms: pct_ms(&s.dec, 0.95),
            cvt_p95_ms: pct_ms(&s.cvt, 0.95),
            recv_p50_ms: pct_i64(&s.recv, 0.5),
            recv_p95_ms: pct_i64(&s.recv, 0.95),
        }
    }

    /// eyepiece_stream.py-compatible line; PC-side extras at the end.
    pub fn line(&self) -> String {
        let mut m = format!(
            "sent {:5.1} fps {:5.1} Mbit/s | shown {:5.1} fps | capture→shown p50 {:5.1} ms p95 {:5.1} ms \
             (capture→send {:4.1}) | decode {:4.1} ms convert {:4.1} ms | queue≤{} dropped {} | pose {:5.1} Hz | rtt {:.1} ms",
            self.sent_fps,
            self.mbps,
            self.shown_fps,
            self.lat_p50_ms,
            self.lat_p95_ms,
            self.cap_send_p50_ms,
            self.dec_p50_ms,
            self.cvt_p50_ms,
            self.q_max,
            self.dropped,
            self.pose_hz,
            self.rtt_ms
        );
        m += &format!(
            " | pc: cap {:5.1} fps, conv {:4.1} enc {:4.1} ms, cap→send p95 {:4.1}, skip {} idr {}",
            self.captured_fps, self.cap_enc_p50_ms, self.enc_p50_ms, self.cap_send_p95_ms, self.skipped, self.idr
        );
        if let Some((y, p)) = self.yaw_pitch {
            m += &format!(" | yaw {y:6.1} pitch {p:6.1}");
        }
        m
    }
}
