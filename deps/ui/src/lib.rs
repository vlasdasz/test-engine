#![allow(incomplete_features)]
#![allow(clippy::single_component_path_imports)]
#![feature(specialization)]
#![feature(const_trait_impl)]
#![feature(arbitrary_self_types)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(unsized_const_params)]
#![feature(const_type_name)]

mod has_data;
mod images;
mod input;
mod layout;
pub mod mobile;
mod modal_view;
mod navigation_view;
mod style;
mod text_field_constraint;
mod to_label;
mod touch_layer;
mod touch_stack;
pub mod ui_dispatch;
mod ui_event;
mod ui_manager;
mod view;
mod views;
mod with_header;

pub use has_data::*;
pub use images::*;
pub use input::*;
pub use layout::*;
pub use modal_view::*;
pub use navigation_view::*;
pub use style::*;
pub use text_field_constraint::*;
pub use to_label::*;
pub use touch_stack::*;
pub use ui_event::*;
pub use ui_manager::*;
pub use ui_proc::*;
pub use view::*;
pub use views::*;
pub use with_header::*;

extern crate core;
pub extern crate ui_proc;

pub const MICROSECONDS_IN_ONE_SECOND: i64 = 1_000_000;
