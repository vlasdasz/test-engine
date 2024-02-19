pub mod checked_convert;
pub mod lossy_convert;

pub trait Abs {
    fn abs(self) -> Self;
}

impl Abs for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Abs for u8 {
    fn abs(self) -> Self {
        self
    }
}

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
}

impl Zero for u8 {
    fn zero() -> Self {
        0
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
