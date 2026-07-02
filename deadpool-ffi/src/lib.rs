use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::sync::OnceLock;

use deadpool_core::Pool;

// ── Last-error storage (per thread) ──────────────────────────────────────────

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn set_last_error(msg: &str) {
    LAST_ERROR.with(|e| *e.borrow_mut() = CString::new(msg).ok());
}

fn clear_last_error() {
    LAST_ERROR.with(|e| *e.borrow_mut() = None);
}

/// Returns a pointer to the last error message, or NULL if no error occurred.
/// The pointer is valid until the next FFI call on this thread.
#[no_mangle]
pub extern "C" fn deadpool_last_error() -> *const c_char {
    LAST_ERROR.with(|e| {
        e.borrow()
            .as_ref()
            .map_or(std::ptr::null(), |s| s.as_ptr())
    })
}

// ── Version ───────────────────────────────────────────────────────────────────

/// Returns a static NUL-terminated string with the library version.
#[no_mangle]
pub extern "C" fn deadpool_version() -> *const c_char {
    static V: OnceLock<CString> = OnceLock::new();
    V.get_or_init(|| CString::new(deadpool_core::APP_VERSION).unwrap())
        .as_ptr()
}

// ── Pool lifecycle ────────────────────────────────────────────────────────────

/// Allocates a new, empty Pool. Caller must free with `deadpool_pool_free`.
#[no_mangle]
pub extern "C" fn deadpool_pool_new() -> *mut Pool {
    Box::into_raw(Box::new(Pool::new()))
}

/// Frees a Pool previously allocated with `deadpool_pool_new`.
///
/// # Safety
/// `pool` must be a valid pointer returned by `deadpool_pool_new`, or NULL.
#[no_mangle]
pub unsafe extern "C" fn deadpool_pool_free(pool: *mut Pool) {
    if !pool.is_null() {
        drop(unsafe { Box::from_raw(pool) });
    }
}

/// Returns 1 if the pool has no characters, 0 otherwise.
///
/// # Safety
/// `pool` must be a valid non-null pointer.
#[no_mangle]
pub unsafe extern "C" fn deadpool_pool_is_empty(pool: *const Pool) -> c_int {
    if pool.is_null() {
        return 1;
    }
    if unsafe { &*pool }.is_empty() { 1 } else { 0 }
}

// ── Charset extension helpers ─────────────────────────────────────────────────

macro_rules! ffi_extend {
    ($fn_name:ident, $method:ident) => {
        /// # Safety
        /// `pool` must be a valid non-null pointer.
        #[no_mangle]
        pub unsafe extern "C" fn $fn_name(pool: *mut Pool) {
            if !pool.is_null() {
                unsafe { &mut *pool }.$method();
            }
        }
    };
}

ffi_extend!(deadpool_extend_lowercase,   extend_from_lowercase);
ffi_extend!(deadpool_extend_uppercase,   extend_from_uppercase);
ffi_extend!(deadpool_extend_digits,      extend_from_digits);
ffi_extend!(deadpool_extend_braces,      extend_from_braces);
ffi_extend!(deadpool_extend_punctuation, extend_from_punctuation);
ffi_extend!(deadpool_extend_quotes,      extend_from_quotes);
ffi_extend!(deadpool_extend_dashes,      extend_from_dashes);
ffi_extend!(deadpool_extend_math,        extend_from_math);
ffi_extend!(deadpool_extend_logograms,   extend_from_logograms);

// ── Include / Exclude ─────────────────────────────────────────────────────────

/// Adds characters from a NUL-terminated string to the pool.
/// Returns 0 on success, -1 on error (call `deadpool_last_error` for the message).
///
/// # Safety
/// `pool` and `chars` must be valid non-null pointers.
#[no_mangle]
pub unsafe extern "C" fn deadpool_extend_from_string(
    pool: *mut Pool,
    chars: *const c_char,
) -> c_int {
    if pool.is_null() || chars.is_null() {
        set_last_error("null pointer");
        return -1;
    }
    let s = match unsafe { CStr::from_ptr(chars) }.to_str() {
        Ok(s) => s,
        Err(e) => { set_last_error(&e.to_string()); return -1; }
    };
    match unsafe { &mut *pool }.extend_from_string(s) {
        Ok(()) => { clear_last_error(); 0 }
        Err(e) => { set_last_error(&e.to_string()); -1 }
    }
}

/// Marks characters in `chars` as excluded from generation.
///
/// # Safety
/// `pool` and `chars` must be valid non-null pointers.
#[no_mangle]
pub unsafe extern "C" fn deadpool_exclude_chars(pool: *mut Pool, chars: *const c_char) {
    if pool.is_null() || chars.is_null() {
        return;
    }
    if let Ok(s) = unsafe { CStr::from_ptr(chars) }.to_str() {
        unsafe { &mut *pool }.exclude_chars(s);
    }
}

// ── Generation ────────────────────────────────────────────────────────────────

/// Generates a password of `length` characters into `out_buf` (`out_len` bytes).
/// Returns 0 on success, -1 on error.
///
/// # Safety
/// `pool` must be a valid non-null pointer. `out_buf` must point to a buffer of
/// at least `out_len` bytes.
#[no_mangle]
pub unsafe extern "C" fn deadpool_generate(
    pool: *const Pool,
    length: usize,
    out_buf: *mut c_char,
    out_len: usize,
) -> c_int {
    if pool.is_null() || out_buf.is_null() || out_len == 0 {
        set_last_error("null pointer or zero buffer");
        return -1;
    }
    match unsafe { &*pool }.generate(length) {
        Ok(pw) => {
            let bytes = pw.as_bytes();
            if bytes.len() + 1 > out_len {
                set_last_error("output buffer too small");
                return -1;
            }
            let dst = unsafe { std::slice::from_raw_parts_mut(out_buf.cast::<u8>(), out_len) };
            dst[..bytes.len()].copy_from_slice(bytes);
            dst[bytes.len()] = 0;
            clear_last_error();
            0
        }
        Err(e) => { set_last_error(&e.to_string()); -1 }
    }
}
