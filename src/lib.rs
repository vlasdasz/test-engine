
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

#[macro_use] mod utils;
#[macro_use] extern crate guard;

mod gm;
mod te;
mod ui;
mod image;
#[macro_use] mod gl_wrapper;

use crate::gm::Size;
use crate::te::Screen;

#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    log!("KOK!");
    log!("KOKOSOK!");
    log!("suehoh!");

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
