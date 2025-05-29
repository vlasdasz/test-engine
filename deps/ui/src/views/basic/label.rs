use std::fmt::Display;

use gm::{
    ToF32,
    color::{BLACK, CLEAR, Color, WHITE},
};
use refs::Weak;
use ui_proc::view;
use window::image::ToImage;

use crate::{
    HasText, ImageView, Setup, Style, ToLabel, View, ViewFrame,
    view::{ViewData, ViewSubviews},
};

mod test_engine {
    pub(crate) use educe;
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

    #[educe(Default = 5.0)]
    pub margin: f32,

    pub text: String,

    pub multiline: bool,

    text_color: Color,
    text_size:  f32,
}

impl HasText for Label {
    fn text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        self.text = text.to_label();
        self
    }

    fn text_color(&self) -> &Color {
        &self.text_color
    }

    fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.text_color = color.into();
        self
    }

    fn text_size(&self) -> f32 {
        self.text_size
    }

    fn set_text_size(&mut self, size: impl ToF32) -> &mut Self {
        self.text_size = size.to_f32();
        self
    }
}

impl Label {
    pub fn set_alignment(&mut self, alignment: TextAlignment) -> &mut Self {
        self.alignment = alignment;
        self
    }

    pub fn set_image(&mut self, image: impl ToImage) -> &mut Self {
        self.set_color(CLEAR);
        self.remove_all_subviews();
        let mut image_view = self.add_view::<ImageView>();
        image_view.place().back();
        image_view.set_image(image);
        image_view.base_view_mut().z_position = self.z_position();

        self
    }

    pub fn set_resizing_image(&mut self, name: impl Display) -> &mut Self {
        self.remove_all_subviews();
        let mut image_view = self.add_view::<ImageView>();
        image_view.place().back();
        image_view.set_resizing_image(name);
        image_view.base_view_mut().z_position = self.z_position();
        image_view.subviews_mut().iter_mut().for_each(|v| {
            v.base_view_mut().z_position = self.z_position();
            v.subviews_mut().iter_mut().for_each(|v| {
                v.base_view_mut().z_position = self.z_position();
            });
        });

        self
    }
}

impl Setup for Label {
    fn setup(mut self: Weak<Self>) {
        self.text_size = 32.0;
        self.set_color(WHITE);
        self.text_color = BLACK;

        Style::apply_global(self);
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
