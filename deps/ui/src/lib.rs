#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(specialization)]
#![feature(const_trait_impl)]
#![feature(arbitrary_self_types)]

pub use input::Touch;

pub mod input;
pub mod layout;
mod modal_view;
mod navigation_view;
mod path_data;
mod shaders;
mod ui_drawer;
mod ui_manager;
mod view;

pub use modal_view::*;
pub use navigation_view::*;
pub use path_data::*;
pub use refs;
pub use shaders::*;
pub use ui_drawer::*;
pub use ui_manager::*;
pub use ui_proc::*;
pub use vents::*;
pub use view::*;

extern crate core;
pub extern crate ui_proc;
