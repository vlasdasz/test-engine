#![allow(clippy::mismatched_target_os)]
#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(specialization)]

pub use input::Touch;

pub mod input;
pub mod layout;
mod path_data;
mod ui_drawer;
mod ui_manager;
mod view;

pub use path_data::*;
pub use refs;
pub use ui_drawer::*;
pub use ui_manager::*;
pub use ui_proc::*;
pub use vents::*;
pub use view::*;

extern crate core;
pub extern crate ui_proc;
