use gl_wrapper::{Buffer, BufferConfig};
use gm::{flat::PointsPath, Color};
use ui::{view, DrawMode, PathData};

#[view]
#[derive(Default)]
pub struct DrawingView {}

impl DrawingView {
    pub fn add_path(&mut self, path: impl Into<PointsPath>, color: &Color, mode: DrawMode) -> &mut Self {
        self.view.paths.push(initialize_path_data(path.into(), color, mode));
        self
    }

    pub fn remove_all_paths(&mut self) {
        self.view.paths.clear()
    }
}

pub fn initialize_path_data(path: PointsPath, color: &Color, draw_mode: DrawMode) -> PathData {
    let buffer = Buffer::make(&BufferConfig::_2, (&path.points).into(), None, draw_mode.to_gl());

    PathData {
        buffer,
        path,
        color: *color,
        draw_mode,
    }
}
