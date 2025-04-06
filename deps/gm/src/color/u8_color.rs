use crate::{
    ToF32,
    color::{Color, helpers::srgb_to_linear},
};

pub type U8Color = Color<u8>;

impl U8Color {
    pub const fn const_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

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
            srgb_to_linear(f32::from(value.r) / 255.0),
            srgb_to_linear(f32::from(value.g) / 255.0),
            srgb_to_linear(f32::from(value.b) / 255.0),
            srgb_to_linear(f32::from(value.a) / 255.0),
        )
    }
}

impl<R: ToF32, G: ToF32, B: ToF32> From<(R, G, B)> for Color {
    fn from(value: (R, G, B)) -> Self {
        Color::rgba(
            srgb_to_linear(value.0.to_f32() / 255.0),
            srgb_to_linear(value.1.to_f32() / 255.0),
            srgb_to_linear(value.2.to_f32() / 255.0),
            1.0,
        )
    }
}
