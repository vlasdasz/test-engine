use std::str::FromStr;

use anyhow::bail;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TouchEvent {
    Began,
    Moved,
    Ended,
}

impl ToString for TouchEvent {
    fn to_string(&self) -> String {
        match self {
            TouchEvent::Began => '↓',
            TouchEvent::Moved => '→',
            TouchEvent::Ended => '↑',
        }
        .to_string()
    }
}

impl FromStr for TouchEvent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "↓" => Ok(TouchEvent::Began),
            "→" => Ok(TouchEvent::Moved),
            "↑" => Ok(TouchEvent::Ended),
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
