#![allow(dead_code)]

use std::os::raw::c_float;
use std::ptr;

mod gl_wrapper;
mod gm;
mod image;
mod te;
mod ui;

use crate::gl_wrapper::gl_wrapper::Updatable;
use crate::gm::Size;
use crate::te::Screen;
use crate::te::ui::TestModel;
use crate::tools::HasNew;

#[macro_use]
extern crate tools;
#[macro_use]
extern crate guard;
#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_use]
extern crate mashup;

static mut SCREEN: *mut Screen<TestModel> = ptr::null_mut();

#[no_mangle]
pub extern "C" fn create_screen() {
    unsafe {
        let mut screen = Screen::<TestModel>::new();
        screen.init();
        SCREEN = Box::into_raw(Box::new(screen));
    }
}

#[no_mangle]
pub extern "C" fn set_screen_size(width: c_float, height: c_float) {
    unsafe {
        SCREEN.as_mut().unwrap().set_size(Size { width, height });
    }
}

#[no_mangle]
pub extern "C" fn update_screen() {
    unsafe {
        SCREEN.as_mut().unwrap().update();
    }
}
