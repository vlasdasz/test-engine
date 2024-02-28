use gm::flat::Point;
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

    pub changed: Event<Point>,
}

impl PointView {
    pub fn point(&self) -> Point {
        (self.x.value(), self.y.value()).into()
    }
}

impl ViewSetup for PointView {
    fn setup(self: Weak<Self>) {
        self.place().all_hor().all(10);

        [self.x, self.y].apply(move |v| {
            v.on_change(move |_| self.changed.trigger(self.point()));
        });
    }
}
