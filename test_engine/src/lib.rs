use std::{
    os::raw::{c_float, c_int, c_ulong},
    ptr,
};

pub use gl_image;
pub use gl_wrapper;
pub use gm;
use gm::Size;
pub use tools;
use tools::new;
pub use ui;

pub use crate::screen::Screen;

mod assets;
pub mod debug_view;
pub mod paths;
pub mod screen;
mod sprites_drawer;
pub mod ui_drawer;

pub use gl_image::Image;
pub use sprites::{Level, LevelBase, Sprite};
use ui::{input::touch::Event, Touch};

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_use]
extern crate mashup;

static mut SCREEN: *mut Screen = ptr::null_mut();

#[no_mangle]
pub extern "C" fn create_screen() {
    unsafe {
        SCREEN = Box::into_raw(Box::new(Screen::new(new())));
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
