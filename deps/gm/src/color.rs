use rtools::{IntoF32, Random};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn is_visible(&self) -> bool {
        self.a > 0.02
    }

    pub fn with_alpha(&self, alpha: impl IntoF32) -> Self {
        (self.r, self.g, self.b, alpha.into_f32()).into()
    }
}

impl Color {
    pub const BLACK: Color = (0, 0, 0).into();
    pub const WHITE: Color = (1, 1, 1).into();
    pub const RED: Color = (1, 0, 0).into();
    pub const GREEN: Color = (0, 1, 0).into();
    pub const BLUE: Color = (0, 0, 0.8).into();
    pub const LIGHT_BLUE: Color = (0, 0.7, 1).into();
    pub const YELLOW: Color = (1, 1, 0).into();
    pub const ORANGE: Color = (1, 0.6, 0).into();
    pub const PURPLE: Color = (1, 0, 1).into();
    pub const TURQUOISE: Color = (0, 1, 1).into();
    pub const GRAY: Color = (0.5, 0.5, 0.5).into();
    pub const BROWN: Color = (0.7, 0.4, 0.2).into();
    pub const LIGHT_GRAY: Color = (0.8, 0.8, 0.8).into();
    pub const CLEAR: Color = (0, 0, 0, 0).into();

    pub const ALL: [Color; 12] = [
        Color::BLACK,
        Color::WHITE,
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::LIGHT_BLUE,
        Color::YELLOW,
        Color::ORANGE,
        Color::PURPLE,
        Color::TURQUOISE,
        Color::BROWN,
        Color::LIGHT_GRAY,
    ];

    pub fn random() -> Color {
        Color::ALL[usize::random_in(0..Color::ALL.len())]
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::CLEAR
    }
}

impl<R, G, B> const From<(R, G, B)> for Color
where
    R: ~const IntoF32,
    G: ~const IntoF32,
    B: ~const IntoF32,
{
    fn from(t: (R, G, B)) -> Self {
        Self {
            r: t.0.into_f32(),
            g: t.1.into_f32(),
            b: t.2.into_f32(),
            a: 1.0,
        }
    }
}

impl<R, G, B, A> const From<(R, G, B, A)> for Color
where
    R: ~const IntoF32,
    G: ~const IntoF32,
    B: ~const IntoF32,
    A: ~const IntoF32,
{
    fn from(t: (R, G, B, A)) -> Self {
        Self {
            r: t.0.into_f32(),
            g: t.1.into_f32(),
            b: t.2.into_f32(),
            a: t.3.into_f32(),
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("r: {}, g: {}, b: {}, a: {}", self.r, self.g, self.b, self.a)
    }
}
