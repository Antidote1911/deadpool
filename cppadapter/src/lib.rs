use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use deadpool::*;

#[no_mangle]
pub extern "C" fn get_version() -> *mut c_char {
    rust_to_c_string(getversion().to_string())
}

#[no_mangle]
pub extern "C" fn get_random(length: u8,
                             uppercase: bool,
                             lowercase:bool,
                             digits:bool,
                             braces:bool,
                             punctuation:bool,
                             quotes:bool,
                             dashes:bool,
                             math:bool,
                             logograms:bool,
                             avoid: *mut c_char,
                             also: *mut c_char) -> *mut c_char {

    let mut pool = deadpool::Pool::new();
    if uppercase {
        pool.extend_from_uppercase();
    }
    if lowercase {
        pool.extend_from_lowercase();
    }
    if digits {
        pool.extend_from_digits();
    }
    if braces {
        pool.extend_from_braces();
    }
    if punctuation {
        pool.extend_from_punctuation();
    }
    if quotes {
        pool.extend_from_quotes();
    }
    if dashes {
        pool.extend_from_dashes();
    }
    if math {
        pool.extend_from_math();
    }
    if logograms {
        pool.extend_from_logograms();
    }

    pool.extend_from_string(c_to_rust_string(also).unwrap().as_str());
    pool.exclude_chars(c_to_rust_string(avoid).unwrap().as_str());

    let password = pool.generate(length as usize).unwrap();
    rust_to_c_string(password)
}


fn rust_to_c_string(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

fn c_to_rust_string(ptr: *mut c_char) -> Result<String, String> {
    let c_str: &CStr = unsafe { CStr::from_ptr(ptr) };
    let res = c_str
        .to_str()
        .map_err(|_| "Could not convert C string to Rust string".to_string())?
        .to_string();
    Ok(res)
}
