mod text;
mod window;
mod window_events;

mod frame_counter;
pub mod image;
mod screenshot;
mod state;
mod surface;
mod vertex_buffer;

pub use bytemuck::cast_slice;
pub use image_proc::include_images;
pub use screenshot::*;
pub use state::RGBA_TEXTURE_FORMAT;
pub use text::*;
pub use vertex_buffer::VertexBuffer;
pub use wgpu::{
    Buffer, BufferUsages, Device, PolygonMode, RenderPass,
    util::{BufferInitDescriptor, DeviceExt},
};
pub use window::*;
pub use window_events::*;
pub use winit::{
    event::{ElementState, MouseButton},
    keyboard::NamedKey,
};
