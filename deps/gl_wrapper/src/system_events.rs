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

#[derive(SmartDefault)]
pub struct SystemEvents {
    pub frame_drawn:  Event,
    pub cursor_moved: Event<Point>,
    pub size_changed: Event<IntSize>,
    pub mouse_click:  Event<(MouseButton, Action)>,
    pub key_pressed:  Event<(Key, Action)>,
    pub scroll:       Event<Point>,
    pub file_drop:    Event<Vec<PathBuf>>,

    #[default(AtomicI32::new(i32::MIN))]
    pub terminate: AtomicI32,
}

impl SystemEvents {
    pub fn get() -> &'static Self {
        &EVENTS
    }

    pub fn terminate(code: ExitCode) {
        EVENTS.terminate.store(code.to_i32(), Ordering::Relaxed);
    }
}
