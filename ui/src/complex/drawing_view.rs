use crate::complex::PathData;
use crate::{View, ViewBase};
use gm::flat::PointsPath;
use gm::Color;
use std::any::Any;
use tools::{new, AsAny, New};

#[derive(Debug)]
pub struct DrawingView {
    base: ViewBase,
    pub paths: Vec<PathData>,
}

impl DrawingView {
    pub fn add_path(&mut self, path: PointsPath, color: Color) {
        // self.paths
        //     .push(UIDrawer::initialize_path_data(path, color, DrawMode::Fill))
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

impl AsAny for DrawingView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl New for DrawingView {
    fn new() -> Self {
        Self {
            base: new(),
            paths: vec![],
        }
    }
}
