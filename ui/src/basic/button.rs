use gl_image::Image;
use tools::{Event, ToRglica};

use crate::{View, ViewBase};

#[derive(Default)]
pub struct Button {
    base:       ViewBase,
    pub on_tap: Event,
    pub image:  Option<Image>,
}

impl View for Button {
    fn setup(&mut self) {
        self.enable_touch();
        let mut this = self.to_rglica();
        self.on_touch().subscribe(move |touch| {
            if touch.is_began() {
                this.on_tap.trigger(());
            }
        });
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
