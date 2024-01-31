use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    ops::{Add, Sub},
};

use bytemuck::{Pod, Zeroable};
use rtools::Random;
use serde::{Deserialize, Serialize};

use crate::num::{Abs, Zero};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorBase<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> ColorBase<T> {
    pub const fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl<T: Copy> ColorBase<T> {
    pub const fn as_slice(&self) -> [T; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn with_alpha(&self, alpha: T) -> Self {
        Self::rgba(self.r, self.g, self.b, alpha)
    }
}

impl<T: Copy + Abs + Sub<Output = T> + Add<Output = T>> ColorBase<T> {
    pub fn diff(&self, other: ColorBase<T>) -> T {
        (self.r - other.r).abs()
            + (self.g - other.g).abs()
            + (self.b - other.b).abs()
            + (self.a - other.a).abs()
    }
}

impl<T: Display> Display for ColorBase<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r: {}, g: {}, b: {}, a: {}", self.r, self.g, self.b, self.a)
    }
}

impl<T: Zero> Default for ColorBase<T> {
    fn default() -> Self {
        Self::rgba(T::zero(), T::zero(), T::zero(), T::zero())
    }
}

pub type Color = ColorBase<f32>;
unsafe impl Zeroable for Color {}
unsafe impl Pod for Color {}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    pub fn is_clear(&self) -> bool {
        !self.is_visible()
    }

    pub fn is_visible(&self) -> bool {
        self.a > 0.02
    }
}

impl Color {
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 0.8);
    pub const LIGHT_BLUE: Color = Color::rgb(0.0, 0.7, 1.0);
    pub const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
    pub const ORANGE: Color = Color::rgb(1.0, 0.6, 0.0);
    pub const PURPLE: Color = Color::rgb(1.0, 0.0, 1.0);
    pub const TURQUOISE: Color = Color::rgb(0.0, 1.0, 1.0);
    pub const GRAY: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const BROWN: Color = Color::rgb(0.7, 0.4, 0.2);
    pub const LIGHT_GRAY: Color = Color::rgb(0.8, 0.8, 0.8);
    pub const LIGHTER_GRAY: Color = Color::rgb(0.9, 0.9, 0.9);
    pub const CLEAR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

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

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.r.to_bits());
        state.write_u32(self.g.to_bits());
        state.write_u32(self.b.to_bits());
        state.write_u32(self.a.to_bits());
        state.finish();
    }
}

impl Eq for Color {}

pub type U8Color = ColorBase<u8>;
unsafe impl Zeroable for U8Color {}
unsafe impl Pod for U8Color {}

#[test]
fn color_diff() {
    assert_eq!(Color::WHITE.diff(Color::CLEAR), 4.0);
    assert_eq!(Color::WHITE.diff(Color::WHITE), 0.0);
    assert_eq!(Color::WHITE.diff(Color::WHITE.with_alpha(0.9)), 0.100000024);
}
