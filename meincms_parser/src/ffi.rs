use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::{markdown, wikitext};

#[no_mangle]
pub unsafe extern "C" fn meincms_markdown_to_html(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    let c_str = CStr::from_ptr(input);
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("").unwrap().into_raw(),
    };

    let html = markdown::to_html(str_slice);
    match CString::new(html) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => CString::new("").unwrap().into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn meincms_markdown_get_categories(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return CString::new("[]").unwrap().into_raw();
    }
    let c_str = CStr::from_ptr(input);
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("[]").unwrap().into_raw(),
    };

    let categories = markdown::get_categories(str_slice);
    let json = serde_json::to_string(&categories).unwrap_or_else(|_| "[]".to_string());
    match CString::new(json) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => CString::new("[]").unwrap().into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn meincms_wikitext_to_html(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    let c_str = CStr::from_ptr(input);
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("").unwrap().into_raw(),
    };

    let html = wikitext::to_html(str_slice);
    match CString::new(html) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => CString::new("").unwrap().into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn meincms_wikitext_get_categories(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return CString::new("[]").unwrap().into_raw();
    }
    let c_str = CStr::from_ptr(input);
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("[]").unwrap().into_raw(),
    };

    let categories = wikitext::get_categories(str_slice);
    let json = serde_json::to_string(&categories).unwrap_or_else(|_| "[]".to_string());
    match CString::new(json) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => CString::new("[]").unwrap().into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn meincms_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}
