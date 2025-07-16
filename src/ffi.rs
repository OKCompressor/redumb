# ffi.rs

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use anyhow::Result;

/// Return 0 on success, non-zero on error.
#[no_mangle]
pub extern "C" fn redumb_encode(
    input: *const c_char,
    dict_dir: *const c_char,
    sdict_dir: *const c_char,
    enc_dir: *const c_char,
) -> c_int {
    wrap(|| {
        let i  = cstr(input)?;
        let d  = cstr(dict_dir)?;
        let sd = cstr(sdict_dir)?;
        let e  = cstr(enc_dir)?;
        redumb::process::transform_chunked(i, d, sd, e)
    })
}

#[no_mangle]
pub extern "C" fn redumb_restore(
    dict_dir: *const c_char,
    enc_dir: *const c_char,
    output: *const c_char,
) -> c_int {
    wrap(|| {
        let d = cstr(dict_dir)?;
        let e = cstr(enc_dir)?;
        let o = cstr(output)?;
        redumb::process::restore_chunked(d, e, o)
    })
}

// ---------- helpers ----------
fn cstr(ptr: *const c_char) -> Result<&'static str> {
    if ptr.is_null() { anyhow::bail!("null ptr") }
    unsafe { Ok(CStr::from_ptr(ptr).to_str()?) }
}

fn wrap(f: impl FnOnce() -> Result<()>) -> c_int {
    match f() { Ok(_) => 0, Err(_) => 1 }
}

