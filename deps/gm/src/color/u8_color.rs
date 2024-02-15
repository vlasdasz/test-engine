use bytemuck::{Pod, Zeroable};

use crate::{Color, ColorBase};

pub type U8Color = ColorBase<u8>;
unsafe impl Zeroable for U8Color {}
unsafe impl Pod for U8Color {}

impl From<U8Color> for Color {
    fn from(value: U8Color) -> Self {
        Color::rgba(
            f32::from(value.r) / 256.0,
            f32::from(value.g) / 256.0,
            f32::from(value.b) / 256.0,
            f32::from(value.a) / 256.0,
        )
    }
}
