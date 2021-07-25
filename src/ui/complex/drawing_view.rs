use crate::gm::flat::PointsPath;
use crate::gm::Color;
use crate::te::UIDrawer;
use crate::ui::complex::path_data::DrawMode;
use crate::ui::complex::PathData;
use crate::ui::{View, ViewBase};
use std::any::Any;
use tools::{new, AsAny, New};

#[derive(Debug)]
pub struct DrawingView {
    base: ViewBase,
    pub paths: Vec<PathData>,
}

impl DrawingView {
    pub fn add_path(&mut self, path: PointsPath, color: Color, draw_mode: DrawMode) {
        self.paths
            .push(UIDrawer::initialize_path_data(path, color, draw_mode))
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
