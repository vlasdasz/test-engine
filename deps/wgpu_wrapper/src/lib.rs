#![feature(const_fn_floating_point_arithmetic)]

mod app;
pub mod image;
mod text;
pub mod utils;
mod wgpu_app;

mod frame_counter;
mod render;
mod screenshot;

pub use app::*;
pub use bytemuck::cast_slice;
pub use render::{
    image_state::image_vertices_with_shrink, path_data::PathData, state::State, wgpu_drawer::WGPUDrawer,
};
pub use screenshot::*;
pub use text::*;
pub use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, PolygonMode,
};
pub use wgpu_app::*;
pub use winit::event::{ElementState, MouseButton};
