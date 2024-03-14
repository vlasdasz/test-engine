#![cfg(ios)]

use std::ffi::{c_char, c_float};

extern "C" {
    pub fn ios_init_text_field();
    pub fn ios_open_keyboard(x: c_float, y: c_float, width: c_float, height: c_float);
    pub fn ios_close_keyboard() -> *const c_char;
}
