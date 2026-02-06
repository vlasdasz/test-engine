use gm::{ToF32, color::Color};

use crate::ToLabel;

pub trait HasText {
    fn text(&self) -> &str;
    fn set_text(&self, text: impl ToLabel) -> &Self;

    fn text_color(&self) -> &Color;
    fn set_text_color(&self, color: impl Into<Color>) -> &Self;

    fn text_size(&self) -> f32;
    fn set_text_size(&self, size: impl ToF32) -> &Self;
}
