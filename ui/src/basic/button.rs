use gl_image::Image;
use proc_macro::{AsAny, Boxed};
use tools::{Event, Rglica};

use crate::{View, ViewBase};

#[derive(AsAny, Boxed)]
pub struct Button {
    base:       ViewBase,
    pub on_tap: Event,
    pub image:  Option<Image>,
}

impl View for Button {
    fn setup(&mut self) {
        self.enable_touch();
        let mut this = Rglica::from_ref(self);
        self.on_touch().subscribe(move |touch| {
            if touch.is_began() {
                this.on_tap.trigger(&());
            }
        });
    }

    fn image(&self) -> Option<Image> { self.image }

    fn view(&self) -> &ViewBase { &self.base }

    fn view_mut(&mut self) -> &mut ViewBase { &mut self.base }
}
