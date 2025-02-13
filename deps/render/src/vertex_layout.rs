use std::mem::size_of;

use gm::flat::{Point, Vertex2D};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

pub trait VertexLayout: Sized {
    const ATTRIBS: &'static [VertexAttribute];
    const VERTEX_LAYOUT: VertexBufferLayout<'static>;
}

impl VertexLayout for Point {
    const ATTRIBS: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0 => Float32x2];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Vertex,
        attributes:   Self::ATTRIBS,
    };
}

impl VertexLayout for Vertex2D {
    const ATTRIBS: &'static [VertexAttribute] = &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Vertex,
        attributes:   Self::ATTRIBS,
    };
}
