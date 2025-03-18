use refs::{Own, Weak};
use window::image::Image;

use crate::object::Object;

#[derive(Default)]
pub struct Game {
    pub objects:    Vec<Own<Object>>,
    pub background: Weak<Image>,
}

impl Game {
    pub fn update(&mut self) {
        for obj in &mut self.objects {
            obj.update();
        }
    }
}
