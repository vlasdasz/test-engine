use crate::Color;

pub type U8Color = Color<u8>;

impl From<U8Color> for Color {
    fn from(value: U8Color) -> Self {
        Color::rgba(
            f32::from(value.r) / 255.0,
            f32::from(value.g) / 255.0,
            f32::from(value.b) / 255.0,
            f32::from(value.a) / 255.0,
        )
    }
}
