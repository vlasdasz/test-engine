
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

mod te;
mod ui;
mod gm;
mod image;
mod gl_wrapper;

use crate::gm::Color;

#[macro_use] extern crate tools;
#[macro_use] extern crate guard;

pub const GL_COLOR_BUFFER_BIT: u32 = 16384;
pub const GL_DEPTH_BUFFER_BIT: u32 = 256;

pub type GLfloat = f32;
pub type GLbitfield = ::std::os::raw::c_uint;

extern "C" {
    pub fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    pub fn glClear(mask: GLbitfield);
}

#[no_mangle]
pub extern fn clear_with_random_color() {

    let color = Color::random();

    unsafe {
        glClearColor(color.r, color.g, color.b, color.a);
        glClear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

}

#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    log!("KOK!");
    log!("KOKOSOK!");
    log!("suehoh! 22");

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
