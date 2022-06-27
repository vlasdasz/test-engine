#![cfg(desktop)]

use glfw::{Action, Key, MouseButton};
use gm::flat::{Point, Size};
use rtools::{static_get, Event};

#[derive(Default)]
pub struct GlEvents {
    pub on_frame_drawn:  Event,
    pub on_cursor_moved: Event<Point>,
    pub on_size_changed: Event<Size>,
    pub on_mouse_click:  Event<(MouseButton, Action)>,
    pub on_key_pressed:  Event<(Key, Action)>,
}

static_get!(GlEvents);
