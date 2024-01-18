#![cfg(desktop)]

use std::{path::PathBuf, sync::Mutex};

use anyhow::Result;
use glfw::{Action, Key, MouseButton};
use gm::flat::{IntSize, Point};
use refs::MainLock;
use smart_default::SmartDefault;
use vents::Event;

static EVENTS: MainLock<SystemEvents> = MainLock::new();
static TERMINATE: Mutex<Option<Result<()>>> = Mutex::new(None);

#[derive(SmartDefault)]
pub struct SystemEvents {
    pub frame_drawn:  Event,
    pub after_draw:   Event,
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

    pub fn terminate(result: Result<()>) {
        *TERMINATE.lock().unwrap() = result.into()
    }

    pub fn check_terminate() -> Option<Result<()>> {
        let mut ter = TERMINATE.lock().unwrap();
        if let Some(ter) = ter.take() {
            return ter.into();
        }
        None
    }
}
