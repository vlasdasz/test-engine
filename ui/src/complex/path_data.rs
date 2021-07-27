use gm::flat::PointsPath;
use gm::Color;

#[derive(Debug)]
pub struct PathData {
    // pub buffer: Buffer,
    pub path: PointsPath,
    pub color: Color,
    pub draw_mode: DrawMode,
}

impl PathData {}

#[derive(Debug)]
pub enum DrawMode {
    Outline,
    Fill,
}
