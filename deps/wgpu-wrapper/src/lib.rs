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
mod vertex_buffer;

pub use app::*;
pub use bytemuck::cast_slice;
pub use image_proc::include_images;
pub use render::{
    flat::RectPipeline, image_drawer::image_vertices_with_shrink, path_data::PathData,
    sprite_drawer::shader_data::SpriteRenderView, wgpu_drawer::WGPUDrawer,
};
pub use screenshot::*;
pub use text::*;
pub use utils::DeviceHelper;
pub use vertex_buffer::VertexBuffer;
pub use wgpu::{
    Buffer, BufferUsages, Device, PolygonMode, RenderPass,
    util::{BufferInitDescriptor, DeviceExt},
};
pub use wgpu_app::*;
pub use winit::{
    event::{ElementState, MouseButton},
    keyboard::NamedKey,
};
