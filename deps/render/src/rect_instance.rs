use bytemuck::{Pod, Zeroable};
use gm::{
    Color,
    flat::{Point, Rect, Size},
};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};
use window::VertexLayout;

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct RectInstance {
    pub origin:     Point,
    pub size:       Size,
    pub color:      Color,
    pub z_position: f32,
    _padding:       u32,
}

impl RectInstance {
    pub fn new(rect: Rect, color: Color, z_position: f32) -> Self {
        Self {
            origin: rect.origin,
            size: rect.size,
            color,
            z_position,
            _padding: 0,
        }
    }
}

impl VertexLayout for RectInstance {
    const ATTRIBS: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![2 => Float32x2, 3 => Float32x2, 4 => Float32x4, 5 => Float32, 6 => Float32];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Instance,
        attributes:   Self::ATTRIBS,
    };
}
