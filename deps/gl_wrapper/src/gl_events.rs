#![cfg(desktop)]

use glfw::{Action, Key, MouseButton};
use gm::flat::{Point, Size};
use rtools::static_default;
use vents::Event;

#[derive(Default)]
pub struct GlEvents {
    pub frame_drawn:  Event,
    pub cursor_moved: Event<Point>,
    pub size_changed: Event<Size>,
    pub mouse_click:  Event<(MouseButton, Action)>,
    pub key_pressed:  Event<(Key, Action)>,
    pub scroll:       Event<Point>,
}

static_default!(GlEvents);
