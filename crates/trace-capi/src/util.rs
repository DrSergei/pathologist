//! FFI plumbing: panic guard, error-string channel, result arena, and the
//! string-free entry point.

use crate::types::TraceStatus;
use std::ffi::{c_char, CStr, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};

/// Run `body`, catching Rust panics so none cross the C boundary. Domain
/// errors come back as messages; panics become `"internal panic: …"`.
pub(crate) fn guard<T>(body: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    match catch_unwind(AssertUnwindSafe(body)) {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(msg)) => Err(msg),
        Err(payload) => {
            let detail = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "unknown panic payload".to_string());
            Err(format!("internal panic: {detail}"))
        }
    }
}

/// Map a domain error message to a status code. Panics (tagged by `guard`
/// with a prefix) map to `TraceErrPanic`; file I/O failures (tagged by the
/// index preflight) map to `TraceErrIo`; argument errors detected inside a
/// `guard` body (tagged with a prefix) map to `TraceErrInvalidArg`; `not
/// found`-family messages map to `TraceErrNotFound` so C consumers can branch
/// on absence. Everything else is `TraceErrAnalysis`.
pub(crate) fn status_for(msg: &str) -> i32 {
    if msg.starts_with("internal panic:") {
        TraceStatus::TraceErrPanic as i32
    } else if msg.starts_with("i/o error:") {
        TraceStatus::TraceErrIo as i32
    } else if msg.starts_with("invalid argument:") {
        TraceStatus::TraceErrInvalidArg as i32
    } else if msg.contains("not found") || msg.contains("no value-flow node") {
        TraceStatus::TraceErrNotFound as i32
    } else {
        TraceStatus::TraceErrAnalysis as i32
    }
}

/// Write `msg` into `*out_err` (C-owned; freed with `trace_string_free`).
/// No-op when `out_err` is null.
pub(crate) unsafe fn set_error(out_err: *mut *mut c_char, msg: &str) {
    if out_err.is_null() {
        return;
    }
    if let Ok(cs) = CString::new(msg) {
        *out_err = cs.into_raw();
    }
}

/// Null `*out_err` at the start of a call so a stale pointer from a previous
/// call can never be observed or double-freed by a caller that only checks
/// `*out_err != NULL`. No-op when `out_err` is null.
pub(crate) unsafe fn reset_err(out_err: *mut *mut c_char) {
    if !out_err.is_null() {
        *out_err = std::ptr::null_mut();
    }
}

/// Read a NUL-terminated C string. The returned reference is valid for the
/// caller's input buffer; callers copy it into owned data before returning.
pub(crate) unsafe fn cstr<'a>(s: *const c_char) -> Result<&'a str, String> {
    if s.is_null() {
        return Err("null string argument".to_string());
    }
    CStr::from_ptr(s)
        .to_str()
        .map_err(|e| format!("argument is not valid UTF-8: {e}"))
}

/// Read a C array of C strings (`argv`-style) into owned `String`s.
pub(crate) unsafe fn str_array(arr: *const *const c_char, n: usize) -> Result<Vec<String>, String> {
    if n == 0 {
        return Ok(Vec::new());
    }
    if arr.is_null() {
        return Err("string array is null with length > 0".to_string());
    }
    let slice = std::slice::from_raw_parts(arr, n);
    let mut out = Vec::with_capacity(n);
    for &p in slice {
        out.push(cstr(p)?.to_owned());
    }
    Ok(out)
}

/// Append-only store of NUL-terminated C strings. Each `CString` is heap
/// allocated once and its buffer is never moved, so pointers handed out by
/// `add` stay valid until the arena is dropped. Dropping the arena frees
/// every string it holds.
#[derive(Default)]
pub(crate) struct Arena {
    strings: Vec<CString>,
}

impl Arena {
    pub fn new() -> Self {
        Self::default()
    }

    /// Copy `s` into the arena and return a stable `char*`.
    pub fn add(&mut self, s: &str) -> *const c_char {
        let cs = CString::new(s).unwrap_or_else(|_| CString::new("<invalid>").unwrap());
        let ptr = cs.as_ptr();
        self.strings.push(cs);
        ptr
    }

    /// Copy an optional string; `None` maps to a null pointer.
    pub fn add_opt(&mut self, s: Option<&str>) -> *const c_char {
        match s {
            Some(v) => self.add(v),
            None => std::ptr::null(),
        }
    }
}

/// Free a string previously returned by this library (error messages).
/// Safe on null; passing any other pointer is undefined behavior.
///
/// # Safety
///
/// The pointer must come from `into_raw` of a `CString` produced here.
#[no_mangle]
pub unsafe extern "C" fn trace_string_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    drop(CString::from_raw(s));
}
