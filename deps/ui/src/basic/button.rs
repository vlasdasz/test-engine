use gl_image::Image;
use rtools::{data_manager::Handle, Event, Rglica};

use crate::{view::ViewSetters, view_base::ViewBase, Label, Touch, View};

#[derive(Default, Debug)]
pub struct Button {
    base:  ViewBase,
    image: Handle<Image>,
    label: Rglica<Label>,

    pub on_tap: Event,
}

impl Button {
    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        if self.label.is_null() {
            self.label = self.add_view();
        }
        self.label.set_text(text);
        self
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

    fn image(&self) -> Handle<Image> {
        self.image
    }

    fn set_image(&mut self, image: Handle<Image>) {
        self.image = image
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
