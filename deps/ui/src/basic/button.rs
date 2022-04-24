use rtools::{Event, Rglica};

use crate::{
    view::{ViewFrame, ViewSubviews},
    Label, View, ViewBase, ViewTouch,
};

#[derive(Default, Debug)]
pub struct Button {
    base:  ViewBase,
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
        self.on_touch().set(self, |touch, this| {
            if touch.is_began() {
                this.on_tap.trigger(())
            }
        });
    }

    fn layout(&mut self) {
        if self.label.is_ok() {
            self.label.place().as_background()
        }
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
