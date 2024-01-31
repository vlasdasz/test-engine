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
pub mod num;
pub mod sign;
pub mod volume;

pub use color::*;
