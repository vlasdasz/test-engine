use bytemuck::{Pod, Zeroable};
use gm::{
    color::Color,
    flat::{Point, Size},
};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::vertex_layout::VertexLayout;

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct UIGradientInstance {
    pub position:      Point,
    pub size:          Size,
    pub start_color:   Color,
    pub end_color:     Color,
    pub corner_radius: f32,
    pub z_position:    f32,
    pub scale:         f32,
}

impl VertexLayout for UIGradientInstance {
    const ATTRIBS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![2 => Float32x2, 3 => Float32x2, 4 => Float32x4, 5 => Float32x4, 6 => Float32, 7 => Float32, 8 => Float32];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Instance,
        attributes:   Self::ATTRIBS,
    };
}
