use std::ops::AddAssign;

use gl_image::Image;
use rtools::{Event, Rglica, ToRglica};

use crate::{
    basic::Button,
    view_base::{init_view_on, ViewBase},
    Label, View,
};

#[derive(Default)]
pub struct IntView {
    base:          ViewBase,
    value:         i64,
    label:         Rglica<Label>,
    up:            Rglica<Button>,
    down:          Rglica<Button>,
    pub on_change: Event<i64>,
}

impl IntView {
    pub fn set_images(&mut self, up: Image, down: Image) {
        self.up.set_image(up);
        self.down.set_image(down);
    }
}

impl View for IntView {
    fn setup(&mut self) {
        self.label = init_view_on(self);
        self.up = init_view_on(self);
        self.down = init_view_on(self);

        let mut this = self.to_rglica();
        self.up.on_tap.subscribe(move |_| {
            this.value.add_assign(1);
            let val = this.value;
            this.on_change.trigger(val);
        });

        let mut this = self.to_rglica();
        self.down.on_tap.subscribe(move |_| {
            this.value.add_assign(-1);
            let val = this.value;
            this.on_change.trigger(val);
        });
    }

    fn layout(&mut self) {
        self.place().subviews_vertically();
        self.label.set_text(&self.value.to_string());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
