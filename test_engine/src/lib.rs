#![allow(dead_code)]

mod ui;
mod paths;
mod screen;
mod assets;
mod sprites;

use std::os::raw::{c_float, c_int, c_ulong};
use std::ptr;
use crate::screen::Screen;
use gm::Size;
use ::ui::input::touch::Event;
use tools::New;


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
        // SCREEN.as_mut().unwrap().on_touch(Touch {
        //     id: id.into(),
        //     position: (x * 2.0, y * 2.0).into(),
        //     event: Event::from_int(event),
        // })
    }
}
