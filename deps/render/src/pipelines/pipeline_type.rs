use std::{marker::ConstParamTy, ops::Range};

use gm::{checked_usize_to_u32, flat::Point};

const VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

pub(super) const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

const TEXTURED_VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[derive(ConstParamTy, PartialEq, Eq)]
pub enum PipelineType {
    Color,
    Image,
}

impl PipelineType {
    pub(crate) const fn color(&self) -> bool {
        matches!(self, PipelineType::Color)
    }

    pub(crate) const fn image(&self) -> bool {
        matches!(self, Self::Image)
    }

    pub(crate) const fn vertex_range(&self) -> Range<u32> {
        match self {
            Self::Color => VERTEX_RANGE,
            Self::Image => TEXTURED_VERTEX_RANGE,
        }
    }
}
