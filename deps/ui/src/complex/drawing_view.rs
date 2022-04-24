use gl_wrapper::{Buffer, BufferConfig};
use gm::{flat::PointsPath, Color};

use crate::{
    complex::{path_data::DrawMode, PathData},
    View, ViewBase,
};

#[derive(Default, Debug)]
pub struct DrawingView {
    base: ViewBase,
}

impl DrawingView {
    pub fn add_path(&mut self, path: impl Into<PointsPath>, color: Color) {
        self.base
            .paths
            .push(initialize_path_data(path.into(), color, DrawMode::Fill))
    }

    pub fn remove_all_paths(&mut self) {
        self.base.paths.clear()
    }
}

impl View for DrawingView {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

fn initialize_path_data(path: PointsPath, color: Color, draw_mode: DrawMode) -> PathData {
    let buffer = Buffer::make(&BufferConfig::_2, (&path.points).into(), None, draw_mode.to_gl());

    PathData {
        buffer,
        path,
        color,
        draw_mode,
    }
}
