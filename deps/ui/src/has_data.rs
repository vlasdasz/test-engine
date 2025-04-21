use gm::{ToF32, color::Color};

use crate::ToLabel;

pub trait HasText {
    fn text(&self) -> &str;
    fn set_text(&mut self, text: impl ToLabel) -> &mut Self;

    fn text_color(&self) -> &Color;
    fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self;

    fn text_size(&self) -> f32;
    fn set_text_size(&mut self, size: impl ToF32) -> &mut Self;
}
