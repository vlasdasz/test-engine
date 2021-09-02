use crate::View;
use gm::Rect;
use proc_macro::New;
use tools::Rglica;

#[derive(New)]
pub struct Placer {
    frame: Rglica<Rect>,
    super_frame: Rglica<Rect>
}

impl Placer {
    pub fn make(view: &Box<dyn View>) -> Self {
        Self {
            frame: Rglica::from_ref(view.frame()),
            super_frame: Rglica::from_ref(view.super_frame())
        }
    }
}

impl Placer {
    pub fn at_center(&mut self) {
       self.frame.origin.x = self.super_frame.width() / 2.0 - self.super_frame.width() / 2.0;
       self.frame.origin.y = self.super_frame.height() / 2.0 - self.super_frame.height() / 2.0;
    }
}
