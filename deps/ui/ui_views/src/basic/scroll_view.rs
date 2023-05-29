use refs::Weak;
use ui::{layout::Anchor, view, Container, SubView, ViewCallbacks, ViewFrame, ViewSetup};

use crate::Slider;

#[view]
pub struct ScrollView {
    slider: SubView<Slider>,
    scroll: SubView<Container>,
}

impl ViewSetup for ScrollView {
    fn setup(self: Weak<Self>) {
        self.scroll.place.relative(Anchor::Height, 1, self);
    }
}

impl ViewCallbacks for ScrollView {
    fn update(&mut self) {
        let co = *self.content_offset();
        self.scroll.frame_mut().origin -= co;
    }
}
