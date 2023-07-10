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
    pub const fn rgb<R, G, B>(r: R, g: G, b: B) -> Self
    where
        R: ~const IntoF32,
        G: ~const IntoF32,
        B: ~const IntoF32, {
        Self {
            r: r.into_f32(),
            g: g.into_f32(),
            b: b.into_f32(),
            a: 1.0,
        }
    }

    pub fn is_clear(&self) -> bool {
        !self.is_visible()
    }

    pub fn is_visible(&self) -> bool {
        self.a > 0.02
    }

    pub fn with_alpha(&self, alpha: impl IntoF32) -> Self {
        (self.r, self.g, self.b, alpha.into_f32()).into()
    }
}

impl Color {
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(1, 1, 1);
    pub const RED: Color = Color::rgb(1, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 1, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 0.8);
    pub const LIGHT_BLUE: Color = Color::rgb(0, 0.7, 1);
    pub const YELLOW: Color = Color::rgb(1, 1, 0);
    pub const ORANGE: Color = Color::rgb(1, 0.6, 0);
    pub const PURPLE: Color = Color::rgb(1, 0, 1);
    pub const TURQUOISE: Color = Color::rgb(0, 1, 1);
    pub const GRAY: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const BROWN: Color = Color::rgb(0.7, 0.4, 0.2);
    pub const LIGHT_GRAY: Color = Color::rgb(0.8, 0.8, 0.8);
    pub const LIGHTER_GRAY: Color = Color::rgb(0.9, 0.9, 0.9);
    pub const CLEAR: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

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

impl<R, G, B> From<(R, G, B)> for Color
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

impl<R, G, B, A> From<(R, G, B, A)> for Color
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
