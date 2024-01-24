use gm::{flat::Points, Color};

use crate::{Buffer, BufferConfig};

#[derive(Debug)]
pub struct PathData {
    pub buffer:    Buffer,
    pub path:      Points,
    pub color:     Color,
    pub draw_mode: DrawMode,
}

#[derive(Debug)]
pub enum DrawMode {
    Outline,
    Fill,
}

impl DrawMode {
    pub fn to_gl(&self) -> u32 {
        match self {
            Self::Outline => 2, //GL_LINE_LOOP
            Self::Fill => 6,    //GL_TRIANGLE_FAN
        }
    }
}

pub fn initialize_path_data(path: Points, color: &Color, draw_mode: DrawMode) -> PathData {
    let float_slice: &[f32] =
        unsafe { std::slice::from_raw_parts(path.as_ptr().cast::<f32>(), path.len() * 2) };

    let buffer = Buffer::make(&BufferConfig::_2, float_slice, None, draw_mode.to_gl());

    PathData {
        buffer,
        path,
        color: *color,
        draw_mode,
    }
}
