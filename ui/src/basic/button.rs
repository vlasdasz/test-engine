use gl_image::Image;
use rtools::{Event, Rglica};

use crate::{
    view_base::{add_view, ViewBase},
    Label, Touch, View,
};

#[derive(Default, Debug)]
pub struct Button {
    base:  ViewBase,
    image: Option<Image>,
    label: Rglica<Label>,

    pub on_tap: Event,
}

impl Button {
    pub fn set_text(&mut self, text: impl ToString) {
        if self.label.is_null() {
            self.label = add_view(self)
        }

        self.label.set_text(text)
    }
}

impl View for Button {
    fn setup(&mut self) {
        self.enable_touch()
    }

    fn layout(&mut self) {
        if self.label.is_ok() {
            self.label.place().as_background()
        }
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_began() {
            self.on_tap.trigger(())
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
