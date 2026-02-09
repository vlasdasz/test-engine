use std::{fmt::Display, sync::atomic::Ordering};

use atomic_float::AtomicF32;
use gm::{
    ToF32,
    color::{BLACK, CLEAR, Color, WHITE},
};
use refs::{Weak, weak_from_ref};
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

static DEFAULT_TEXT_SIZE: AtomicF32 = AtomicF32::new(16.0);

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

    multiline: bool,

    #[educe(Default = BLACK)]
    text_color: Color,

    text_size: f32,
}

impl HasText for Label {
    fn text(&self) -> &str {
        &self.text
    }

    fn set_text(&self, text: impl ToLabel) -> &Self {
        weak_from_ref(self).text = text.to_label();
        self
    }

    fn text_color(&self) -> &Color {
        &self.text_color
    }

    fn set_text_color(&self, color: impl Into<Color>) -> &Self {
        weak_from_ref(self).text_color = color.into();
        self
    }

    fn text_size(&self) -> f32 {
        self.text_size
    }

    fn set_text_size(&self, size: impl ToF32) -> &Self {
        weak_from_ref(self).text_size = size.to_f32();
        self
    }
}

impl Label {
    pub fn set_alignment(&self, alignment: TextAlignment) -> &Self {
        weak_from_ref(self).alignment = alignment;
        self
    }

    pub fn is_multiline(&self) -> bool {
        self.multiline
    }

    pub fn set_multiline(&self, multiline: bool) -> &Self {
        weak_from_ref(self).multiline = multiline;
        self
    }

    pub fn set_image(&self, image: impl ToImage) -> &Self {
        self.set_color(CLEAR);
        self.remove_all_subviews();
        let image_view = self.add_view::<ImageView>();
        image_view.place().back();
        image_view.set_image(image);
        image_view.__base_view().z_position = self.z_position();

        self
    }

    pub fn set_resizing_image(&mut self, name: impl Display) -> &mut Self {
        self.set_color(CLEAR);
        self.remove_all_subviews();
        let mut image_view = self.add_view::<ImageView>();
        image_view.place().back();
        image_view.set_resizing_image(name);
        image_view.__base_view().z_position = self.z_position();
        image_view.subviews_mut().iter_mut().for_each(|v| {
            v.__base_view().z_position = self.z_position();
            v.subviews_mut().iter_mut().for_each(|v| {
                v.__base_view().z_position = self.z_position();
            });
        });

        self
    }
}

impl Label {
    pub fn set_default_text_size(size: impl ToF32) {
        DEFAULT_TEXT_SIZE.store(size.to_f32(), Ordering::Relaxed);
    }
}

impl Setup for Label {
    fn setup(mut self: Weak<Self>) {
        self.text_size = DEFAULT_TEXT_SIZE.load(Ordering::Relaxed);
        self.set_color(WHITE);

        Style::apply_global(self);
    }
}

pub trait AddLabel {
    fn add_label(&self, text: impl ToLabel) -> &Self;
}

impl<T: ?Sized + View> AddLabel for T {
    fn add_label(&self, text: impl ToLabel) -> &Self {
        let mut label = self.add_view::<Label>();
        label.place().center().h(20).lr(0);
        label.text = text.to_label();
        self
    }
}
