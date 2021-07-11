use crate::ui::ViewBase;
use crate::ui::view::View;
use tools::{HasNew, AsAny, Event};
use std::any::Any;
use tools::refs::Shared;

#[derive(Debug)]
pub struct Button {
    base: ViewBase,
    pub on_tap: Event<()>
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

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl HasNew for Button {
    fn new() -> Self where Self: Sized {
        Button {
            base: ViewBase::new(),
            on_tap: Event::new()
        }
    }
}

impl AsAny for Button {
    fn as_any(&self) -> &dyn Any {
        self
    }
}