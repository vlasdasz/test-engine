use std::mem::size_of;

use gm::{flat::Point, volume::UIVertex};

pub trait VertexLayout: Sized {
    const ATTRIBS: &'static [wgpu::VertexAttribute];
    fn vertex_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode:    wgpu::VertexStepMode::Vertex,
            attributes:   Self::ATTRIBS,
        }
    }
}

impl VertexLayout for Point {
    const ATTRIBS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![0 => Float32x2];
}

impl VertexLayout for UIVertex {
    const ATTRIBS: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];
}
