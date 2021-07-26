#![allow(dead_code)]

use std::os::raw::{c_float, c_int, c_ulong};
use std::ptr;

pub mod gl_wrapper;
pub mod gm;
pub mod image;
pub mod sprites;
pub mod te;
pub mod ui;

use crate::gm::Size;
use crate::te::Screen;
use crate::tools::New;
use crate::ui::input::touch::Event;
use crate::ui::input::Touch;

#[macro_use]
extern crate tools;
#[macro_use]
extern crate guard;
#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_use]
extern crate mashup;

static mut SCREEN: *mut Screen = ptr::null_mut();

#[no_mangle]
pub extern "C" fn create_screen() {
    unsafe {
        let mut screen = Screen::new();
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
            id,
            position: (x * 2.0, y * 2.0).into(),
            event: Event::from_int(event),
        })
    }
}
