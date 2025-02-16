#![allow(incomplete_features)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]

pub use crate::shader_data::SpriteView;
mod buffer_helper;
mod device_helper;
mod path_data;
mod pipelines;
pub mod rect_instance;
pub mod rect_view;
mod shader_data;
mod to_bytes;
pub mod ui_rect_instance;
mod uniform;
mod vec_buffer;
mod vertex_layout;

pub use path_data::PathData;
pub use pipelines::*;
