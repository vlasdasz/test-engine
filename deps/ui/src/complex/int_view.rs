use std::ops::AddAssign;

use gl_image::Image;
use rtools::{data_manager::Handle, Event, Rglica, ToRglica};

use crate::{
    basic::Button,
    view_base::{add_view, ViewBase},
    Label, View,
};

#[derive(Default, Debug)]
pub struct IntView {
    base:          ViewBase,
    value:         i64,
    label:         Rglica<Label>,
    up:            Rglica<Button>,
    down:          Rglica<Button>,
    pub on_change: Event<i64>,
}

impl IntView {
    pub fn set_images(&mut self, up: Handle<Image>, down: Handle<Image>) {
        self.up.set_image(up);
        self.down.set_image(down);
    }
}

impl View for IntView {
    fn setup(&mut self) {
        self.label = add_view(self);
        self.up = add_view(self);
        self.down = add_view(self);

        self.up
            .on_tap
            .subscribe(self.to_rglica(), move |_, mut this| {
                this.value.add_assign(1);
                let val = this.value;
                this.on_change.trigger(val);
            });

        self.down
            .on_tap
            .subscribe(self.to_rglica(), move |_, mut this| {
                this.value.add_assign(-1);
                let val = this.value;
                this.on_change.trigger(val);
            });
    }

    fn layout(&mut self) {
        self.place().all_vertically();
        self.label.set_text(&self.value.to_string());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
