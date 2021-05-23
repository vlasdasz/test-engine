use crate::gm::Point;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use crate::ui::input::touch::MouseButton::Undefined;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::Action;

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Undefined,
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
impl MouseButton {
    pub fn from_glfw(btn: glfw::MouseButton) -> Self {
        match btn {
            glfw::MouseButtonLeft => Self::Left,
            glfw::MouseButtonRight => Self::Right,
            glfw::MouseButtonMiddle => Self::Middle,
            _ => Undefined,
        }
    }
}

pub enum ButtonState {
    Up,
    Down,
    Repeat,
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
impl ButtonState {
    pub fn from_glfw(action: Action) -> Self {
        match action {
            Action::Release => Self::Up,
            Action::Press => Self::Down,
            Action::Repeat => Self::Repeat,
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Began,
    Moved,
    Ended,
}

impl Event {
    pub fn from_state(state: ButtonState) -> Self {
        match state {
            ButtonState::Up => Self::Ended,
            ButtonState::Down => Self::Began,
            ButtonState::Repeat => Self::Moved,
        }
    }
}

#[derive(Debug)]
pub struct Touch {
    pub id: i32,
    pub position: Point,
    pub event: Event,
}
