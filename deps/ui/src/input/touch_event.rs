use std::str::FromStr;

use anyhow::bail;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TouchEvent {
    Began,
    Moved,
    Ended,
}

#[cfg(desktop)]
impl TouchEvent {
    pub fn glfw_action(&self) -> glfw::Action {
        match self {
            TouchEvent::Began => glfw::Action::Press,
            TouchEvent::Moved => glfw::Action::Repeat,
            TouchEvent::Ended => glfw::Action::Release,
        }
    }
}

impl ToString for TouchEvent {
    fn to_string(&self) -> String {
        match self {
            TouchEvent::Began => "b",
            TouchEvent::Moved => "m",
            TouchEvent::Ended => "e",
        }
        .to_string()
    }
}

impl FromStr for TouchEvent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" => Ok(TouchEvent::Began),
            "m" => Ok(TouchEvent::Moved),
            "e" => Ok(TouchEvent::Ended),
            _ => bail!("Failed to parse TouchEvent from {s}"),
        }
    }
}

impl TouchEvent {
    // #[cfg(desktop)]
    // pub fn from_state(state: ButtonState) -> Self {
    //     match state {
    //         ButtonState::Up => Self::Ended,
    //         ButtonState::Down => Self::Began,
    //         ButtonState::Repeat => Self::Moved,
    //     }
    // }
}

impl From<i32> for TouchEvent {
    fn from(value: i32) -> Self {
        match value {
            0 => TouchEvent::Began,
            1 => TouchEvent::Moved,
            2 => TouchEvent::Ended,
            _ => unreachable!("Invalid value for touch event"),
        }
    }
}
