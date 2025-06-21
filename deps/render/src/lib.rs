#![allow(incomplete_features)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]

pub use crate::shader_data::SpriteView;
mod buffer_helper;
pub mod data;
mod device_helper;
mod pipelines;
mod shader_data;
mod to_bytes;
mod uniform;
mod vec_buffer;
mod vertex_layout;

pub use pipelines::*;
