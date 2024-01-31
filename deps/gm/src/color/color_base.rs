use std::{
    fmt::Display,
    ops::{Add, Sub},
};

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

impl<T: PartialEq> Eq for ColorBase<T> {}
