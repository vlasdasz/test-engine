use gl_wrapper::{Buffer, BufferConfig};
use gm::{flat::PointsPath, Color};
use ui::{view, DrawMode, PathData};

#[view]
pub struct DrawingView {
    paths: Vec<PathData>,
}

impl DrawingView {
    pub fn paths(&self) -> &[PathData] {
        &self.paths
    }

    pub fn add_path(&mut self, path: impl Into<PointsPath>, color: &Color, mode: DrawMode) -> &mut Self {
        self.paths.push(initialize_path_data(path.into(), color, mode));
        self
    }

    pub fn remove_all_paths(&mut self) {
        self.paths.clear()
    }
}

pub fn initialize_path_data(path: PointsPath, color: &Color, draw_mode: DrawMode) -> PathData {
    let float_slice: &[f32] =
        unsafe { std::slice::from_raw_parts(path.points.as_ptr().cast::<f32>(), path.points.len() * 2) };

    let buffer = Buffer::make(&BufferConfig::_2, float_slice, None, draw_mode.to_gl());

    PathData {
        buffer,
        path,
        color: *color,
        draw_mode,
    }
}
