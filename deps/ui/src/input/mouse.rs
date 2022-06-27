#![cfg(desktop)]

use glfw::Action;

use crate::input::TouchEvent;

pub enum MouseButtonState {
    Up,
    Down,
    Repeat,
}

impl MouseButtonState {
    pub fn from_glfw(action: Action) -> Self {
        match action {
            Action::Release => Self::Up,
            Action::Press => Self::Down,
            Action::Repeat => Self::Repeat,
        }
    }
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Undefined,
}

impl MouseButton {
    pub fn from_glfw(btn: glfw::MouseButton) -> Self {
        match btn {
            glfw::MouseButtonLeft => Self::Left,
            glfw::MouseButtonRight => Self::Right,
            glfw::MouseButtonMiddle => Self::Middle,
            _ => MouseButton::Undefined,
        }
    }
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
