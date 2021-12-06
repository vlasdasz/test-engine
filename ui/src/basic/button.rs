use gl_image::Image;
use tools::Event;

use crate::{Touch, View, ViewBase};

#[derive(Default)]
pub struct Button {
    base:       ViewBase,
    pub on_tap: Event,
    pub image:  Option<Image>,
}

impl View for Button {
    fn handle_touch(&mut self, touch: &Touch) {
        if touch.is_began() {
            self.on_tap.trigger(());
        }
    }

    fn setup(&mut self) {
        self.enable_touch();
    }

    fn image(&self) -> Option<Image> {
        self.image.clone()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
