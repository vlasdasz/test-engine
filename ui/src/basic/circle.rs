use gm::{flat::PointsPath, Color, Rect};
use tools::Rglica;

use crate::{complex::DrawingView, init_view_on, View, ViewBase};

#[derive(Default)]
pub struct Circle {
    base:    ViewBase,
    drawing: Rglica<DrawingView>,
    color:   Color,
}

impl Circle {
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.drawing.remove_all_paths();
        let frame: Rect = self.frame().square().into();
        self.drawing.set_frame(frame);
        self.drawing.add_path(
            PointsPath::circle_with(frame.size.center(), frame.size.width),
            color,
        );
    }
}

impl View for Circle {
    fn setup(&mut self) {
        self.drawing = init_view_on(self);
    }

    fn layout(&mut self) {
        self.set_color(self.color);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
