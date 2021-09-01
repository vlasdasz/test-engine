use crate::{View, ViewBase};
use gl_image::Image;
use proc_macro::{AsAny, New};
use tools::{refs::Shared, Event, Rglica};

#[derive(AsAny, New)]
pub struct Button {
    base: ViewBase,
    pub on_tap: Event,
    pub image: Option<Image>,
}

impl View for Button {
    fn setup(&mut self) {
        self.enable_touch();
        let this = Rglica::from_ref(self);
        self.on_touch().subscribe(move |touch| {
            if touch.is_began() {
                this.on_tap.trigger(&());
            }
        });
    }

    fn image(&self) -> Option<Image> {
        self.image
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
