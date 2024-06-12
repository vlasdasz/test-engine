use std::{mem::size_of, ops::Range};

use bytemuck::{Pod, Zeroable};
use gm::{
    checked_usize_to_u32,
    flat::{Point, Size},
    Color,
};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::render::vertex_layout::VertexLayout;

pub(super) const FULL_SCREEN_VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

pub(super) const FULL_SCREEN_VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(FULL_SCREEN_VERTICES.len());

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Zeroable, Pod, PartialEq)]
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
    pub color:    Color,
    pub rotation: f32,
    pub paddind:  u32,
}

impl VertexLayout for SpriteInstance {
    const ATTRIBS: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![2 => Float32x2, 3 => Float32x2, 4 => Float32x4, 5 => Float32];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Instance,
        attributes:   Self::ATTRIBS,
    };
}
