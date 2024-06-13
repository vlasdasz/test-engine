#![feature(const_fn_floating_point_arithmetic)]

mod app;
pub mod image;
mod text;
pub mod utils;
mod wgpu_app;

mod frame_counter;
mod render;
mod screenshot;
mod state;
mod surface;

pub use app::*;
pub use bytemuck::cast_slice;
pub use render::{
    image_drawer::image_vertices_with_shrink, path_data::PathData, sprite_drawer::shader_data::SpriteView,
    wgpu_drawer::WGPUDrawer,
};
pub use screenshot::*;
pub use text::*;
pub use utils::DeviceHelper;
pub use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, PolygonMode, RenderPass,
};
pub use wgpu_app::*;
pub use winit::event::{ElementState, MouseButton};
