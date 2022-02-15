use gl_image::Image;
use rtools::Event;

use crate::{view_base::ViewBase, Touch, View};

#[derive(Default)]
pub struct Button {
    base:  ViewBase,
    image: Option<Image>,

    pub on_tap: Event,
}

impl View for Button {
    fn setup(&mut self) {
        self.enable_touch();
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_began() {
            self.on_tap.trigger(());
        }
    }

    fn image(&self) -> Option<Image> {
        self.image.clone()
    }

    fn set_image(&mut self, image: Image) {
        self.image = image.into()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
