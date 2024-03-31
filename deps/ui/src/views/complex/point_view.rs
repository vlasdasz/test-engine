use gm::{flat::Point, Apply, ToF32};
use refs::Weak;
use ui_proc::view;
use vents::Event;

use crate::{view::ViewData, IntView, Sub, ViewSetup};

mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct PointView {
    x: Sub<IntView>,
    y: Sub<IntView>,

    mul: f32,

    pub changed: Event<Point>,
}

impl PointView {
    pub fn point(&self) -> Point {
        (self.x.value(), self.y.value()).into()
    }

    pub fn set_multiplier(&mut self, mul: impl ToF32) -> &mut Self {
        self.mul = mul.to_f32();
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
