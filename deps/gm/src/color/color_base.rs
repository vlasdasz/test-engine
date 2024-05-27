use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::num::{Abs, One, ZeroOrMinimal};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color<T = f32> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

unsafe impl<T: Zeroable> Zeroable for Color<T> {}
unsafe impl<T: Pod> Pod for Color<T> {}

impl<T> Color<T> {
    pub const fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }

    pub fn bgra_to_rgba(self) -> Self {
        Self {
            r: self.b,
            g: self.g,
            b: self.r,
            a: self.a,
        }
    }
}

impl<T: Copy> Color<T> {
    pub const fn as_slice(&self) -> [T; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn with_alpha(&self, alpha: T) -> Self {
        Self::rgba(self.r, self.g, self.b, alpha)
    }
}

impl<T: Copy + Abs + Sub<Output = T> + Add<Output = T>> Color<T> {
    pub fn diff(&self, other: Color<T>) -> T {
        (self.r - other.r).abs()
            + (self.g - other.g).abs()
            + (self.b - other.b).abs()
            + (self.a - other.a).abs()
    }
}

impl<T: One> From<(T, T, T)> for Color<T> {
    fn from(value: (T, T, T)) -> Self {
        Self::rgba(value.0, value.1, value.2, T::one())
    }
}

impl<T> From<(T, T, T, T)> for Color<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self::rgba(value.0, value.1, value.2, value.3)
    }
}

impl<T: Display> Display for Color<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r: {}, g: {}, b: {}, a: {}", self.r, self.g, self.b, self.a)
    }
}

impl<T: ZeroOrMinimal> Default for Color<T> {
    fn default() -> Self {
        Self::rgba(T::zero(), T::zero(), T::zero(), T::zero())
    }
}

impl<T: PartialEq> Eq for Color<T> {}
