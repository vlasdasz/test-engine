use std::ops::AddAssign;

use gl_image::Image;
use proc_macro::Boxed;
use tools::{Event, Rglica};

use crate::{basic::Button, make_view_on, Label, View, ViewBase};

#[derive(Boxed)]
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
        self.up.image = up.into();
        self.down.image = down.into();
    }
}

impl View for IntView {
    fn setup(&mut self) {
        self.label = make_view_on(self);
        self.up = make_view_on(self);
        self.down = make_view_on(self);

        let mut this = Rglica::from_ref(self);
        self.up.on_tap.subscribe(move |_| {
            this.value.add_assign(1);
            let val = this.value;
            this.on_change.trigger(val);
        });

        let mut this = Rglica::from_ref(self);
        self.down.on_tap.subscribe(move |_| {
            this.value.add_assign(-1);
            let val = this.value;
            this.on_change.trigger(val);
        });
    }

    fn update(&mut self) { self.label.set_text(&self.value.to_string()); }

    fn layout(&mut self) { self.place().distribute_vertically(); }

    fn view(&self) -> &ViewBase { &self.base }

    fn view_mut(&mut self) -> &mut ViewBase { &mut self.base }
}
