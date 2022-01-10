#![cfg(not(any(target_os = "ios", target_os = "android")))]

use glfw::{Action, Key, MouseButton};
use gm::{Point, Size};
use tools::Event;

#[derive(Default)]
pub struct Events {
    pub on_frame_drawn:  Event,
    pub on_cursor_moved: Event<Point>,
    pub on_size_changed: Event<Size>,
    pub on_mouse_click:  Event<(MouseButton, Action)>,
    pub on_key_pressed:  Event<(Key, Action)>,
}
