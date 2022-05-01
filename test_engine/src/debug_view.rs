use rtools::{platform::Platform, Property, Rglica, ToRglica};
use ui::{impl_view, view, Label, View, ViewBase, ViewCallbacks, ViewFrame, ViewSubviews};

#[view]
#[derive(Default, Debug)]
pub struct DebugView {
    fps_label:         Rglica<Label>,
    frame_drawn_label: Rglica<Label>,
    frame_drawn:       u64,
    pub fps:           Property<u64>,
}

impl_view!(DebugView);

impl ViewCallbacks for DebugView {
    fn setup(&mut self) {
        self.set_frame((280, 100));

        self.fps_label = self.add_view();
        self.fps_label.set_text("fps label");

        self.frame_drawn_label = self.add_view();
        self.frame_drawn_label.set_text("frame drawn label");

        if Platform::MOBILE {
            self.set_origin((28, 28));
        }

        self.fps.on_set.set(self, |this, _| {
            let fps = this.fps.copy();
            this.fps_label.set_text(format!("FPS: {}", fps));
        });
    }

    fn layout(&mut self) {
        self.place().top_left(10);
        self.place().all_vertically();

        self.frame_drawn += 1;
        self.frame_drawn_label
            .set_text(format!("Frame drawn: {}", self.frame_drawn));
    }
}
