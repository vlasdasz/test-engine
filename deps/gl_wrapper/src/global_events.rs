#![cfg(desktop)]

use std::ptr;

use glfw::{Action, Key, MouseButton};
use gm::flat::{Point, Size};
use rtools::Event;

static mut EVENTS: *const GlobalEvents = ptr::null_mut();

#[derive(Default)]
pub struct GlobalEvents {
    pub on_frame_drawn:  Event,
    pub on_cursor_moved: Event<Point>,
    pub on_size_changed: Event<Size>,
    pub on_mouse_click:  Event<(MouseButton, Action)>,
    pub on_key_pressed:  Event<(Key, Action)>,
}

impl GlobalEvents {
    pub fn get() -> &'static GlobalEvents {
        unsafe {
            if EVENTS.is_null() {
                EVENTS = Box::into_raw(Box::new(GlobalEvents::default()));
            }
            EVENTS.as_ref().unwrap_unchecked()
        }
    }
}
