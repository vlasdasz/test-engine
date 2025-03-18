use gm::flat::{Point, Size};
use refs::Weak;
use window::image::Image;

pub struct Object {
    pub position: Point,
    pub size:     Size,
    pub rotation: f32,

    pub image: Weak<Image>,

    pub velocity: Point,
}

impl Object {
    pub fn update(&mut self) {
        self.position += self.velocity;
    }
}
