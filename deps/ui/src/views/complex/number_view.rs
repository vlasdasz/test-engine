use gm::{
    CheckedSub, MyAdd, ToF32,
    color::{CLEAR, Color, LIGHT_GRAY},
};
use refs::Weak;
use vents::Event;

use crate::{Button, Container, HasText, Setup, Style, ToLabel, UIImages, view::ViewData};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use ui_proc::view;
use window::image::NoImage;

#[view]
pub struct NumberView {
    #[educe(Default = 1.0)]
    value:    f32,
    #[educe(Default = 1.0)]
    pub step: f32,
    #[educe(Default = f32::MIN)]
    min:      f32,

    on_change_event: Event<f32>,

    #[init]
    up:   Button,
    down: Button,

    separator: Container,
}

impl Setup for NumberView {
    fn setup(mut self: Weak<Self>) {
        self.up.set_image(UIImages::up()).set_color(CLEAR);
        self.up.on_tap(move || self.up_tap());
        self.up.place().lrt(0).relative_height(self, 0.5);

        self.down.set_image(UIImages::down()).set_color(CLEAR);
        self.down.on_tap(move || self.down_tap());
        self.down.place().lrb(0).relative_height(self, 0.5);

        self.separator.place().relative_width(self, 0.75).h(2).center();
        self.separator.set_color(LIGHT_GRAY);

        Style::apply_global(self);
    }
}

impl NumberView {
    pub fn set_labels(&mut self, up: impl ToLabel, down: impl ToLabel) -> &mut Self {
        self.up.set_image(NoImage);
        self.up.set_text(up);

        self.down.set_image(NoImage);
        self.down.set_text(down);

        self
    }
}

impl NumberView {
    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, val: impl ToF32) -> &mut Self {
        self.value = val.to_f32();
        self.on_change_event.trigger(self.value);
        self
    }

    pub fn set_min(&mut self, min: impl ToF32) -> &mut Self {
        self.min = min.to_f32();
        self.set_value(self.min);
        self
    }

    pub fn set_step(&mut self, step: impl ToF32) -> &mut Self {
        self.step = step.to_f32();
        self
    }

    fn up_tap(mut self: Weak<Self>) {
        let val = self.value.my_add(&self.step);
        self.set_value(val);
    }

    fn down_tap(mut self: Weak<Self>) {
        let val = self.value.sub_and_check(&self.step, &self.min);
        self.set_value(val.unwrap_or(0.0));
    }

    pub fn on_change(&self, action: impl FnMut(f32) + Send + 'static) -> &Self {
        self.on_change_event.val(action);
        self
    }
}

impl HasText for NumberView {
    fn text(&self) -> &str {
        todo!()
    }

    fn set_text(&mut self, _text: impl ToLabel) -> &mut Self {
        todo!()
    }

    fn text_color(&self) -> &Color {
        self.up.text_color()
    }

    fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self {
        let color = color.into();
        self.up.set_text_color(color);
        self.down.set_text_color(color);
        self
    }

    fn text_size(&self) -> f32 {
        self.up.text_size()
    }

    fn set_text_size(&mut self, size: impl ToF32) -> &mut Self {
        self.up.set_text_size(size);
        self.down.set_text_size(size);
        self
    }
}
