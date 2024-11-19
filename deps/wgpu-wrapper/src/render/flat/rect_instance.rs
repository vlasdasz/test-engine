use bytemuck::{Pod, Zeroable};
use gm::{
    flat::{Point, Size},
    Color,
};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::render::vertex_layout::VertexLayout;

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub(super) struct RectInstance {
    pub origin:     Point,
    pub size:       Size,
    pub color:      Color,
    pub z_position: f32,
    pub _padding:   u32,
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
