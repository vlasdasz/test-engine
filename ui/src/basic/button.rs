use crate::{View, ViewBase};
use gl_image::Image;
use proc_macro::AsAny;
use proc_macro::New;
use tools::refs::Shared;
use tools::Event;

#[derive(Debug, AsAny, New)]
pub struct Button {
    base: ViewBase,
    pub on_tap: Event,
    pub image: Option<Image>,
}

impl View for Button {
    fn setup(&mut self, this: Shared<dyn View>) {
        self.enable_touch();
        let this = this.clone();
        self.on_touch().subscribe(move |touch| {
            if touch.is_began() {
                let this = this.borrow();
                let this = this.as_any().downcast_ref::<Button>().unwrap();
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
