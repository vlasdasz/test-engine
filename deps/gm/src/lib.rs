#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(adt_const_params)]
#![feature(specialization)]

extern crate core;

pub mod axis;
mod color;
pub mod converter;
pub mod flat;
pub mod misc;
mod num;
pub mod sign;
pub mod volume;

pub use color::*;
pub use num::{
    checked_convert::{checked_usize_to_u32, CheckedConvert},
    into_f32::IntoF32,
    lossy_convert::LossyConvert,
};
