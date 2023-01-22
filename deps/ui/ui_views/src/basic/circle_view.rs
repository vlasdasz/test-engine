use gm::{flat::PointsPath, Color};
use refs::Weak;
use rtools::IntoF32;
use ui::{view, DrawMode, SubView, ViewFrame, ViewSetup};

use crate::DrawingView;

#[view]
#[derive(Default)]
pub struct CircleView {
    drawing: SubView<DrawingView>,
    color:   Color,
    radius:  f32,
}

impl CircleView {
    pub fn set_radius(&mut self, radius: impl IntoF32) -> &mut Self {
        self.radius = radius.into_f32();
        self.redraw();
        self
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.redraw();
    }

    fn redraw(&mut self) {
        self.drawing.remove_all_paths();
        let frame = self.frame().with_zero_origin();
        self.drawing.add_path(
            PointsPath::circle_with(frame.size.center(), frame.size.width, 50),
            &self.color,
            DrawMode::Fill,
        );
    }
}

impl ViewSetup for CircleView {
    fn setup(mut self: Weak<Self>) {
        self.radius = 10.0;
        let size = self.radius;
        self.place.size(size, size);
        self.drawing.place.size(size, size);
    }
}
