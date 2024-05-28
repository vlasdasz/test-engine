use std::{cmp::max, num::NonZeroU32};

pub mod checked_convert;
pub mod into_f32;
pub mod lossy_convert;

pub trait Abs {
    fn abs(self) -> Self;
}

impl Abs for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

pub trait ZeroOrMinimal {
    fn zero() -> Self;
}

pub trait IsZero: ZeroOrMinimal + Copy {
    fn is_zero(self) -> bool;
}

impl<T: ZeroOrMinimal + PartialEq + Copy> IsZero for T {
    fn is_zero(self) -> bool {
        self == Self::zero()
    }
}

impl ZeroOrMinimal for usize {
    fn zero() -> Self {
        0
    }
}

impl ZeroOrMinimal for u32 {
    fn zero() -> Self {
        0
    }
}

impl ZeroOrMinimal for f32 {
    fn zero() -> Self {
        0.0
    }
}

impl ZeroOrMinimal for u8 {
    fn zero() -> Self {
        0
    }
}

impl ZeroOrMinimal for NonZeroU32 {
    fn zero() -> Self {
        Self::one()
    }
}

#[const_trait]
pub trait One {
    fn one() -> Self;
}

impl const One for f32 {
    fn one() -> Self {
        1.0
    }
}

impl const One for u32 {
    fn one() -> Self {
        1
    }
}

impl const One for usize {
    fn one() -> Self {
        1
    }
}

impl One for NonZeroU32 {
    fn one() -> Self {
        1.try_into().unwrap()
    }
}

pub trait CheckedSub: Sized {
    fn sub_and_check(&self, other: &Self, min: &Self) -> Option<Self>;
}

impl CheckedSub for u32 {
    fn sub_and_check(&self, other: &Self, min: &Self) -> Option<Self> {
        self.checked_sub(*other).map(|a| max(a, *min))
    }
}

impl CheckedSub for usize {
    fn sub_and_check(&self, other: &Self, min: &Self) -> Option<Self> {
        self.checked_sub(*other).map(|a| max(a, *min))
    }
}

impl CheckedSub for f32 {
    fn sub_and_check(&self, other: &Self, min: &Self) -> Option<Self> {
        let res = self - other;
        if res > *min { *min } else { res }.into()
    }
}

impl CheckedSub for NonZeroU32 {
    fn sub_and_check(&self, other: &Self, min: &Self) -> Option<Self> {
        let val = self.get().sub_and_check(&other.get(), &min.get())?;
        val.try_into().ok()
    }
}

pub trait MyAdd {
    fn my_add(&self, other: &Self) -> Self;
}

impl MyAdd for f32 {
    fn my_add(&self, other: &Self) -> Self {
        self + other
    }
}

impl MyAdd for u32 {
    fn my_add(&self, other: &Self) -> Self {
        self + other
    }
}

impl MyAdd for usize {
    fn my_add(&self, other: &Self) -> Self {
        self + other
    }
}

impl MyAdd for NonZeroU32 {
    fn my_add(&self, other: &Self) -> Self {
        (self.get() + other.get()).try_into().unwrap()
    }
}
