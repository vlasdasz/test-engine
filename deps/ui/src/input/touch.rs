#[cfg(desktop)]
use glfw::Action;
use gm::flat::Point;

#[cfg(desktop)]
use crate::input::touch::MouseButton::Undefined;

#[cfg(desktop)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Undefined,
}

#[cfg(desktop)]
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

#[cfg(desktop)]
impl ButtonState {
    pub fn from_glfw(action: Action) -> Self {
        match action {
            Action::Release => Self::Up,
            Action::Press => Self::Down,
            Action::Repeat => Self::Repeat,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TouchEvent {
    Began,
    Moved,
    Ended,
}

impl TouchEvent {
    pub fn from_state(state: ButtonState) -> Self {
        match state {
            ButtonState::Up => Self::Ended,
            ButtonState::Down => Self::Began,
            ButtonState::Repeat => Self::Moved,
        }
    }

    pub fn from_int(event: i32) -> TouchEvent {
        match event {
            0 => TouchEvent::Began,
            1 => TouchEvent::Moved,
            2 => TouchEvent::Ended,
            _ => unreachable!("Invalid value for touch event"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Touch {
    pub id:       u64,
    pub position: Point,
    pub event:    TouchEvent,
}

impl Touch {
    pub fn is_began(&self) -> bool {
        self.event == TouchEvent::Began
    }

    pub fn is_moved(&self) -> bool {
        self.event == TouchEvent::Moved
    }

    pub fn is_ended(&self) -> bool {
        self.event == TouchEvent::Ended
    }
}
