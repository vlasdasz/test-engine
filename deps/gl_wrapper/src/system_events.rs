#![cfg(desktop)]

use std::{
    path::PathBuf,
    process::ExitCode,
    sync::atomic::{AtomicI32, Ordering},
};

use glfw::{Action, Key, MouseButton};
use gm::flat::{IntSize, Point};
use refs::MainLock;
use smart_default::SmartDefault;
use vents::Event;

static EVENTS: MainLock<SystemEvents> = MainLock::new();
static TERMINATE: AtomicI32 = AtomicI32::new(i32::MIN);

#[derive(SmartDefault)]
pub struct SystemEvents {
    pub frame_drawn:  Event,
    pub cursor_moved: Event<Point>,
    pub size_changed: Event<IntSize>,
    pub mouse_click:  Event<(MouseButton, Action)>,
    pub key_pressed:  Event<(Key, Action)>,
    pub scroll:       Event<Point>,
    pub file_drop:    Event<Vec<PathBuf>>,
}

impl SystemEvents {
    pub fn get() -> &'static Self {
        &EVENTS
    }

    pub fn terminate(code: ExitCode) {
        TERMINATE.store(code.to_i32(), Ordering::Relaxed);
    }

    pub fn check_terminate() -> Option<ExitCode> {
        let terminate = TERMINATE.load(Ordering::Relaxed);
        if terminate == i32::MIN {
            return None;
        }
        Some(u8::try_from(terminate).unwrap().into())
    }
}
