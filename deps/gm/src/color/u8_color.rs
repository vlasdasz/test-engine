use crate::{Color, color::helpers::linear_to_srgb};

pub type U8Color = Color<u8>;

impl U8Color {
    pub fn diff_u8(&self, other: Self) -> i16 {
        (i16::from(self.r) - i16::from(other.r)).abs()
            + (i16::from(self.g) - i16::from(other.g)).abs()
            + (i16::from(self.b) - i16::from(other.b)).abs()
            + (i16::from(self.a) - i16::from(other.a)).abs()
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
