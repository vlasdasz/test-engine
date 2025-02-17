use crate::{Color, color::helpers::linear_to_srgb};

pub type U8Color = Color<u8>;

impl U8Color {
    pub fn diff_u8(&self, other: Self) -> i16 {
        (self.r as i16 - other.r as i16).abs()
            + (self.g as i16 - other.g as i16).abs()
            + (self.b as i16 - other.b as i16).abs()
            + (self.a as i16 - other.a as i16).abs()
    }
}

impl From<U8Color> for Color {
    fn from(value: U8Color) -> Self {
        Color::rgba(
            linear_to_srgb(f32::from(value.r) / 255.0),
            linear_to_srgb(f32::from(value.g) / 255.0),
            linear_to_srgb(f32::from(value.b) / 255.0),
            linear_to_srgb(f32::from(value.a) / 255.0),
        )
    }
}
