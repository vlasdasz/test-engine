use gm::{ToF32, color::Color, flat::PointsPath};
use refs::Weak;
use ui_proc::view;

use crate::{
    DrawingView, Setup,
    view::{ViewData, ViewFrame},
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct CircleView {
    color: Color,

    #[init]
    drawing: DrawingView,
}

impl CircleView {
    pub fn set_radius(&mut self, radius: impl ToF32) -> &mut Self {
        let diameter = radius.to_f32() * 2.0;
        self.set_size(diameter, diameter);
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
        );
    }
}

impl Setup for CircleView {
    fn setup(mut self: Weak<Self>) {
        self.set_size(10, 10);
        self.drawing.place().back();
    }
}
