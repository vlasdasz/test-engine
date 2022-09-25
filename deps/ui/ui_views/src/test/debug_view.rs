use rtools::{platform::Platform, Property};
use ui::{view, SubView, ViewCallbacks, ViewFrame};

use crate::Label;

#[view]
#[derive(Default)]
pub struct DebugView {
    fps_label:         SubView<Label>,
    frame_drawn_label: SubView<Label>,
    frame_drawn:       u64,
    pub fps:           Property<u64>,
}

impl ViewCallbacks for DebugView {
    fn setup(&mut self) {
        self.set_frame((10, 10, 200, 50));
        self.place.all_ver();

        self.fps_label.set_text("fps label");

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
        self.frame_drawn_label.set_text(format!("Frame drawn: {}", self.frame_drawn));
    }
}
