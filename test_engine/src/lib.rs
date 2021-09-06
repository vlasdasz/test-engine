#![allow(dead_code)]

mod assets;
mod paths;
mod sprites;
mod test_screen;
mod ui;

use std::{
    os::raw::{c_float, c_int, c_ulong},
    ptr,
};

use ::ui::{input::touch::Event, Touch};
use gl_wrapper::Screen;
use gm::Size;
use tools::New;

pub use crate::test_screen::TestScreen;

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_use]
extern crate mashup;

static mut SCREEN: *mut TestScreen = ptr::null_mut();

#[no_mangle]
pub extern "C" fn create_screen() {
    unsafe {
        let mut screen = TestScreen::new();
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

#[no_mangle]
pub extern "C" fn on_touch(id: c_ulong, x: c_float, y: c_float, event: c_int) {
    unsafe {
        SCREEN.as_mut().unwrap().on_touch(Touch {
            id:       id.into(),
            position: (x * 2.0, y * 2.0).into(),
            event:    Event::from_int(event),
        })
    }
}
