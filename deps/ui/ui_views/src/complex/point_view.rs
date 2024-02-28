use gm::{flat::Point, IntoF32};
use refs::Weak;
use rtools::Apply;
use ui::{view, SubView, ViewData, ViewSetup};
use vents::Event;

use crate::IntView;

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

#[view]
pub struct PointView {
    x: SubView<IntView>,
    y: SubView<IntView>,

    mul: f32,

    pub changed: Event<Point>,
}

impl PointView {
    pub fn point(&self) -> Point {
        (self.x.value(), self.y.value()).into()
    }

    pub fn set_multiplier(&mut self, mul: impl IntoF32) -> &mut Self {
        self.mul = mul.into_f32();
        self
    }
}

impl ViewSetup for PointView {
    fn setup(mut self: Weak<Self>) {
        self.mul = 1.0;
        self.place().all_hor().all(10);

        [self.x, self.y].apply(move |v| {
            v.on_change(move |_| self.changed.trigger(self.point() * self.mul));
        });
    }
}
