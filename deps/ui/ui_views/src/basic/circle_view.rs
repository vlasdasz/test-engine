use gm::{flat::PointsPath, Color};
use refs::Weak;
use rtools::IntoF32;
use ui::{view, SubView, ViewData, ViewFrame, ViewSetup};
use wgpu_wrapper::PolygonMode;

use crate::DrawingView;

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

#[view]
pub struct CircleView {
    drawing: SubView<DrawingView>,
    color:   Color,
}

impl CircleView {
    pub fn set_radius(&mut self, radius: impl IntoF32) -> &mut Self {
        let diameter = radius.into_f32() * 2.0;
        self.set_size((diameter, diameter));
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
            PointsPath::circle_triangles_with(frame.size.center(), frame.size.width / 2.0, 50),
            self.color,
            PolygonMode::Fill,
        );
    }
}

impl ViewSetup for CircleView {
    fn setup(mut self: Weak<Self>) {
        self.set_size((10, 10));
        self.drawing.place().back();
    }
}
