#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(adt_const_params)]

extern crate core;

mod animation;
pub mod axis;
mod color;
pub mod converter;
pub mod flat;
mod misc;
mod num;
pub mod sign;
pub mod volume;

pub use animation::Animation;
pub use color::*;
pub use misc::{Apply, Platform, Toggle};
pub use num::{
    CheckedSub, IsZero, Min, MyAdd, One, Zero,
    checked_convert::{CheckedConvert, checked_usize_to_u32},
    into_f32::ToF32,
    lossy_convert::LossyConvert,
};
