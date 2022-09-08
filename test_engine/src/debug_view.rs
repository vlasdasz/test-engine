use rtools::{platform::Platform, Property, Rglica, ToRglica};
use ui::{view, Label, View, ViewBase, ViewCallbacks, ViewFrame, ViewLayout, ViewSubviews};

#[view]
#[derive(Default, Debug)]
pub struct DebugView {
    fps_label:         Rglica<Label>,
    frame_drawn_label: Rglica<Label>,
    frame_drawn:       u64,
    pub fps:           Property<u64>,
}

impl ViewCallbacks for DebugView {
    fn setup(&mut self) {
        self.place().top().left().val(10).width(200).height(50).all_ver();

        self.fps_label = self.add_view();
        self.fps_label.set_text("fps label");

        self.frame_drawn_label = self.add_view();
        self.frame_drawn_label.set_text("frame drawn label");

        if Platform::MOBILE {
            self.set_origin((28, 28));
        } else {
            self.set_origin((10, 10));
        }

        self.fps.on_set.set(self, |this, _| {
            let fps = this.fps.copy();
            this.fps_label.set_text(format!("FPS: {}", fps));
        });
    }

    fn update(&mut self) {
        self.frame_drawn += 1;
        self.frame_drawn_label
            .set_text(format!("Frame drawn: {}", self.frame_drawn));
    }
}
