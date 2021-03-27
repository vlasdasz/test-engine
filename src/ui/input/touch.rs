use crate::gm::Point;
use crate::gl_wrapper::gl_drawer::ButtonState;

#[derive(Debug)]
pub enum Event {
    Began,
    Moved,
    Ended
}

impl Event {
    pub fn from_state(state: ButtonState) -> Self {
        match state {
            ButtonState::Up     => Self::Ended,
            ButtonState::Down   => Self::Began,
            ButtonState::Repeat => Self::Moved
        }
    }
}

#[derive(Debug)]
pub struct Touch {
    pub id: i32,
    pub position: Point,
    pub event: Event
}