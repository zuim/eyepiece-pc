//! bindgen output + small helpers.
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code, improper_ctypes, unnecessary_transmutes, clippy::all)]

include!(concat!(env!("OUT_DIR"), "/ffi.rs"));

use std::ffi::{CStr, CString};

pub const fn averror(e: i32) -> i32 {
    -e
}
const fn fferrtag(a: u8, b: u8, c: u8, d: u8) -> i32 {
    -((a as i32) | (b as i32) << 8 | (c as i32) << 16 | (d as i32) << 24)
}
pub const AVERROR_EOF_: i32 = fferrtag(b'E', b'O', b'F', b' ');
pub const EAGAIN_: i32 = averror(libc::EAGAIN);

pub fn err_str(e: i32) -> String {
    let mut buf = [0 as std::ffi::c_char; 256];
    unsafe { av_strerror(e, buf.as_mut_ptr(), buf.len()) };
    unsafe { CStr::from_ptr(buf.as_ptr()) }.to_string_lossy().into_owned()
}

/// Ok(ret) for ret >= 0, else Err("<what>: <av error>").
pub fn check(ret: i32, what: &str) -> Result<i32, String> {
    if ret < 0 {
        Err(format!("{what}: {}", err_str(ret)))
    } else {
        Ok(ret)
    }
}

pub fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

pub fn pix_fmt_name(f: i32) -> String {
    let p = unsafe { av_get_pix_fmt_name(std::mem::transmute::<i32, AVPixelFormat>(f)) };
    if p.is_null() {
        return format!("fmt{f}");
    }
    unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned()
}

/// Owned AVFrame.
pub struct Frame(pub *mut AVFrame);
unsafe impl Send for Frame {}
impl Frame {
    pub fn alloc() -> Frame {
        let f = unsafe { av_frame_alloc() };
        assert!(!f.is_null(), "av_frame_alloc");
        Frame(f)
    }
    pub fn as_ptr(&self) -> *mut AVFrame {
        self.0
    }
    pub fn width(&self) -> i32 {
        unsafe { (*self.0).width }
    }
    pub fn height(&self) -> i32 {
        unsafe { (*self.0).height }
    }
    pub fn format(&self) -> i32 {
        unsafe { (*self.0).format }
    }
}
impl Drop for Frame {
    fn drop(&mut self) {
        unsafe { av_frame_free(&mut self.0) };
    }
}

/// Owned AVBufferRef (device / frames contexts).
pub struct BufRef(pub *mut AVBufferRef);
unsafe impl Send for BufRef {}
impl BufRef {
    pub fn new_ref(&self) -> *mut AVBufferRef {
        unsafe { av_buffer_ref(self.0) }
    }
}
impl Clone for BufRef {
    fn clone(&self) -> Self {
        BufRef(self.new_ref())
    }
}
impl Drop for BufRef {
    fn drop(&mut self) {
        unsafe { av_buffer_unref(&mut self.0) };
    }
}

/// AVDictionary built from key/value pairs; reports keys the codec didn't consume.
pub struct Dict(pub *mut AVDictionary);
impl Dict {
    pub fn new() -> Dict {
        Dict(std::ptr::null_mut())
    }
    pub fn set(&mut self, k: &str, v: &str) {
        unsafe { av_dict_set(&mut self.0, cstr(k).as_ptr(), cstr(v).as_ptr(), 0) };
    }
    pub fn keys(&self) -> Vec<String> {
        let mut out = vec![];
        let mut e: *const AVDictionaryEntry = std::ptr::null();
        loop {
            e = unsafe { av_dict_iterate(self.0, e) };
            if e.is_null() {
                break;
            }
            out.push(unsafe { CStr::from_ptr((*e).key) }.to_string_lossy().into_owned());
        }
        out
    }
}
impl Drop for Dict {
    fn drop(&mut self) {
        unsafe { av_dict_free(&mut self.0) };
    }
}

pub fn set_log_level(level: i32) {
    unsafe { av_log_set_level(level) };
}
