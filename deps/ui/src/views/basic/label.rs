use gm::{Color, ToF32};
use refs::Weak;
use ui_proc::view;

use crate::{
    view::{ViewData, ViewSubviews},
    ToLabel, View, ViewSetup,
};

mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[derive(Debug, Default)]
pub enum TextAlignment {
    Left,
    #[default]
    Center,
    Right,
}

impl TextAlignment {
    pub fn center(&self) -> bool {
        matches!(self, Self::Center)
    }
}

#[view]
pub struct Label {
    pub alignment: TextAlignment,

    pub text:   String,
    text_color: Color,
    text_size:  f32,
}

impl Label {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        self.text = text.to_label();
        self
    }

    pub fn set_alignment(&mut self, alignment: TextAlignment) -> &mut Self {
        self.alignment = alignment;
        self
    }

    pub fn text_color(&self) -> &Color {
        &self.text_color
    }

    pub fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.text_color = color.into();
        self
    }

    pub fn text_size(&self) -> f32 {
        self.text_size
    }

    pub fn set_text_size(&mut self, size: impl ToF32) -> &mut Self {
        self.text_size = size.to_f32();
        self
    }
}

impl ViewSetup for Label {
    fn setup(mut self: Weak<Self>) {
        self.text_size = 32.0;
        self.set_color(Color::WHITE);
        self.text_color = Color::BLACK;
    }
}

pub trait AddLabel {
    fn add_label(&mut self, text: impl ToLabel) -> &mut Self;
}

impl<T: ?Sized + View> AddLabel for T {
    fn add_label(&mut self, text: impl ToLabel) -> &mut Self {
        let mut label = self.add_view::<Label>();
        label.place().center().h(20).lr(0);
        label.text = text.to_label();
        self
    }
}
