#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(associated_type_bounds)]
#![feature(const_trait_impl)]

mod basic;
mod complex;
mod images;
mod test;

pub use basic::*;
pub use complex::*;
pub use images::*;
pub use test::*;
