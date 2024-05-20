use std::{mem::size_of, ops::Range};

use bytemuck::{Pod, Zeroable};
use gm::{
    checked_usize_to_u32,
    flat::{Point, Size},
};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::render::vertex_layout::VertexLayout;

pub(super) const VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

pub(super) const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
pub(super) struct SpriteView {
    pub camera_pos:      Point,
    pub resolution:      Size,
    pub camera_rotation: f32,
    pub scale:           f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub(super) struct SpriteInstance {
    pub size:     Size,
    pub position: Point,
    pub rotation: f32,
    pub paddind:  u32,
}

impl VertexLayout for SpriteInstance {
    const ATTRIBS: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![1 => Float32x2, 2 => Float32x2, 3 => Float32, 4 => Uint32];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Instance,
        attributes:   Self::ATTRIBS,
    };
}
