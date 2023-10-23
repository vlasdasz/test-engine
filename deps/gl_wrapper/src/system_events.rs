#![cfg(desktop)]

use std::{
    path::PathBuf,
    sync::atomic::{AtomicI32, Ordering},
};

use glfw::{Action, Key, MouseButton};
use gm::flat::{Point, Size};
use refs::is_main_thread;
use vents::Event;

static mut EVENTS: *const SystemEvents = std::ptr::null_mut();

pub struct SystemEvents {
    pub frame_drawn:  Event,
    pub cursor_moved: Event<Point>,
    pub size_changed: Event<Size>,
    pub mouse_click:  Event<(MouseButton, Action)>,
    pub key_pressed:  Event<(Key, Action)>,
    pub scroll:       Event<Point>,
    pub file_drop:    Event<Vec<PathBuf>>,
    pub terminate:    AtomicI32,
}

impl SystemEvents {
    fn init() -> Self {
        Self {
            frame_drawn:  Default::default(),
            cursor_moved: Default::default(),
            size_changed: Default::default(),
            mouse_click:  Default::default(),
            key_pressed:  Default::default(),
            scroll:       Default::default(),
            file_drop:    Default::default(),
            terminate:    AtomicI32::new(i32::MIN),
        }
    }

    pub fn get() -> &'static Self {
        debug_assert!(is_main_thread());
        unsafe {
            if EVENTS.is_null() {
                EVENTS = Box::into_raw(Box::new(Self::init()));
            }
            EVENTS.as_ref().unwrap()
        }
    }

    pub fn terminate(code: i32) {
        unsafe { EVENTS.as_ref().unwrap().terminate.store(code, Ordering::Relaxed) };
    }
}
