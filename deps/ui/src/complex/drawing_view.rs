use gl_wrapper::{Buffer, BufferConfig};
use gm::{flat::PointsPath, Color};
use rtools::{Rglica, ToRglica};

use crate::{
    complex::{path_data::DrawMode, PathData},
    impl_view, view, View, ViewBase,
};

#[view]
#[derive(Default, Debug)]
pub struct DrawingView {}
impl_view!(DrawingView);

impl DrawingView {
    pub fn add_path(&mut self, path: impl Into<PointsPath>, color: Color) {
        self.view
            .paths
            .push(initialize_path_data(path.into(), color, DrawMode::Fill))
    }

    pub fn remove_all_paths(&mut self) {
        self.view.paths.clear()
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
