use bytemuck::{Pod, Zeroable};
use gm::{
    Color,
    flat::{Point, Rect, Size},
};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::vertex_layout::VertexLayout;

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct UIRectInstance {
    pub position:      Point,
    pub size:          Size,
    pub color:         Color,
    pub corner_radius: f32,
    pub z_position:    f32,
}

impl UIRectInstance {
    pub fn new(rect: Rect, color: Color, corner_radius: f32, z_position: f32) -> Self {
        Self {
            position: rect.origin,
            size: rect.size,
            color,
            corner_radius,
            z_position,
        }
    }
}

impl VertexLayout for UIRectInstance {
    const ATTRIBS: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![2 => Float32x2, 3 => Float32x2, 4 => Float32x4, 5 => Float32, 6 => Float32];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Instance,
        attributes:   Self::ATTRIBS,
    };
}
