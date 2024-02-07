#![allow(incomplete_features)]
#![allow(clippy::single_component_path_imports)]
#![feature(specialization)]
#![feature(const_trait_impl)]
#![feature(arbitrary_self_types)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

pub use input::Touch;

pub mod input;
mod labeled;
mod layout;
mod modal_view;
mod navigation_view;
mod text_field_constraint;
mod to_label;
mod touch_layer;
mod touch_stack;
mod ui_event;
mod ui_manager;
mod view;
mod with_header;

pub use labeled::*;
pub use layout::*;
pub use modal_view::*;
pub use navigation_view::*;
pub use refs;
pub use text_field_constraint::*;
pub use to_label::*;
pub use touch_stack::*;
pub use ui_event::*;
pub use ui_manager::*;
pub use ui_proc::*;
pub use vents::*;
pub use view::*;
pub use with_header::*;

extern crate core;
pub extern crate ui_proc;

pub const MICROSECONDS_IN_ONE_SECOND: i64 = 1_000_000;
