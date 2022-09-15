#![allow(clippy::mismatched_target_os)]
#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(specialization)]

pub use basic::{ImageView, Label};
pub use complex::DPadView;
pub use input::Touch;

pub mod basic;
pub mod complex;
pub mod input;
pub mod layout;
pub mod test;
mod ui_drawer;
mod view;

pub use ui_drawer::*;
pub use ui_proc::*;
pub use view::*;

extern crate core;
pub extern crate ui_proc;
