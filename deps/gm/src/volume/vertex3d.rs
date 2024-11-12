use bytemuck::{Pod, Zeroable};

use crate::{flat::Point, volume::Point3};

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Zeroable, Pod)]
pub struct Vertex3D {
    pub pos: Point3,
    pub uv:  Point,
}
