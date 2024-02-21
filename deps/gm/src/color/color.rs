use std::hash::{Hash, Hasher};

use rtools::Random;

use crate::{color::helpers::srgb_to_linear, num::lossy_convert::LossyConvert, Color, U8Color};

impl Color<f32> {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    pub fn from_srgb(self) -> Self {
        Self::rgba(
            srgb_to_linear(self.r),
            srgb_to_linear(self.g),
            srgb_to_linear(self.b),
            srgb_to_linear(self.a),
        )
    }
}

impl From<Color> for U8Color {
    fn from(value: Color) -> Self {
        U8Color::rgba(
            (255.0 * value.r).lossy_convert(),
            (255.0 * value.g).lossy_convert(),
            (255.0 * value.b).lossy_convert(),
            (255.0 * value.a).lossy_convert(),
        )
    }
}

impl Color {
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 0.8);
    pub const LIGHT_BLUE: Color = Color::rgb(0.0, 0.7, 1.0);
    pub const GRAY_BLUE: Color = Color::rgb(0.1, 0.2, 0.3);
    pub const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
    pub const ORANGE: Color = Color::rgb(1.0, 0.6, 0.0);
    pub const PURPLE: Color = Color::rgb(1.0, 0.0, 1.0);
    pub const TURQUOISE: Color = Color::rgb(0.0, 1.0, 1.0);
    pub const GRAY: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const BROWN: Color = Color::rgb(0.7, 0.4, 0.2);
    pub const LIGHT_GRAY: Color = Color::rgb(0.8, 0.8, 0.8);
    pub const LIGHTER_GRAY: Color = Color::rgb(0.9, 0.9, 0.9);
    pub const CLEAR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

    const ALL: [Color; 14] = [
        Color::BLACK,
        Color::WHITE,
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::LIGHT_BLUE,
        Color::GRAY_BLUE,
        Color::YELLOW,
        Color::ORANGE,
        Color::PURPLE,
        Color::TURQUOISE,
        Color::BROWN,
        Color::LIGHT_GRAY,
        Color::LIGHTER_GRAY,
    ];

    pub fn random() -> Self {
        Self::ALL[usize::random_in(0..Self::ALL.len())]
    }
}

impl Hash for Color<f32> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.r.to_bits());
        state.write_u32(self.g.to_bits());
        state.write_u32(self.b.to_bits());
        state.write_u32(self.a.to_bits());
        state.finish();
    }
}

#[test]
fn color_diff() {
    assert_eq!(Color::WHITE.diff(Color::CLEAR), 4.0);
    assert_eq!(Color::WHITE.diff(Color::WHITE), 0.0);
    assert_eq!(Color::WHITE.diff(Color::WHITE.with_alpha(0.9)), 0.100000024);
}

#[test]
fn color_to_u8() {
    let color: U8Color = Color::rgba(0.5, 1.0, 0.1, 0.0).into();
    assert_eq!(color, U8Color::rgba(127, 255, 25, 0));
}
