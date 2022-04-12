use gm::{flat::PointsPath, Color};
use rtools::{Boxed, Rglica};

use crate::{
    complex::DrawingView,
    view_base::{add_view_with_frame, ViewBase},
    View,
};

#[derive(Debug)]
pub struct CircleView {
    base:    ViewBase,
    drawing: Rglica<DrawingView>,
    color:   Color,
    radius:  f32,
}

impl CircleView {
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.drawing.remove_all_paths();
        let frame = self.frame().with_zero_origin();
        self.drawing.add_path(
            PointsPath::circle_with(frame.size.center(), frame.size.width),
            color,
        );
    }
}

impl View for CircleView {
    fn setup(&mut self) {
        let size = (self.radius, self.radius);
        self.drawing = add_view_with_frame(self, size);
        self.frame_mut().size = size.into();
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl CircleView {
    pub fn with_radius(radius: f32) -> Box<Self> {
        Box::new(Self {
            base: Default::default(),
            drawing: Default::default(),
            color: Default::default(),
            radius,
        })
    }
}

impl Boxed for CircleView {
    fn boxed() -> Box<Self> {
        panic!("Initialize CircleView using CircleView::with_radius")
    }
}
