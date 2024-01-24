use gl_wrapper::Buffer;
use gm::{flat::Points, Color};

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
