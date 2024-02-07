#![cfg(desktop)]

use crate::input::TouchEvent;

pub enum MouseButtonState {
    Up,
    Down,
    Repeat,
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Undefined,
}

impl From<MouseButtonState> for TouchEvent {
    fn from(state: MouseButtonState) -> Self {
        match state {
            MouseButtonState::Down => TouchEvent::Began,
            MouseButtonState::Up => TouchEvent::Ended,
            MouseButtonState::Repeat => TouchEvent::Moved,
        }
    }
}
