use gm::{flat::PointsPath, Color};
use rtools::{Boxed, Rglica};

use crate::{
    complex::DrawingView,
    impl_view, view,
    view::{ViewFrame, ViewSubviews},
    View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Debug)]
pub struct CircleView {
    drawing: Rglica<DrawingView>,
    color:   Color,
    radius:  f32,
}

impl_view!(CircleView);

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

impl ViewCallbacks for CircleView {
    fn setup(&mut self) {
        let size = (self.radius, self.radius);
        self.drawing = self.add_view_with_frame(size);
        self.set_frame(size);
    }
}

impl CircleView {
    pub fn with_radius(radius: f32) -> Box<Self> {
        Box::new(Self {
            view: Default::default(),
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
