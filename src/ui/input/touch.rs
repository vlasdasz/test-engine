use crate::gm::Point;

pub enum Event {
    Began,
    Moved,
    Ended
}

pub struct Touch {
    pub id: i32,
    pub position: Point,
    pub event: Event
}