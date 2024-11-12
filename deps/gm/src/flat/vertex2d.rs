use bytemuck::{Pod, Zeroable};

use crate::flat::Point;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Zeroable, Pod)]
pub struct Vertex2D {
    pub pos: Point,
    pub uv:  Point,
}
