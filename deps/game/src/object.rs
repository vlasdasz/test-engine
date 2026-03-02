use refs::Weak;
use window::image::Image;

use crate::{Point, Rotation, Shape};

pub struct Object {
    pub position: Point,
    pub velocity: Point,

    pub shape:    Shape,
    pub rotation: Rotation,

    pub texture: Weak<Image>,
}

impl Object {
    pub fn update(&mut self) {
        self.position += self.velocity;
    }
}
