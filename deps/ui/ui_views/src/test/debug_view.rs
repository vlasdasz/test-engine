use refs::ToWeak;
use rtools::{platform::Platform};
use ui::{view, SubView, ViewCallbacks, ViewFrame, Property};

use crate::Label;

#[view]
#[derive(Default)]
pub struct DebugView {
    fps_label:         SubView<Label>,
    frame_drawn_label: SubView<Label>,
    url_label:         SubView<Label>,
    frame_drawn:       u64,
    pub fps:           Property<u64>,
    pub url:           Property<String>,
}

impl ViewCallbacks for DebugView {
    fn setup(&mut self) {
        self.set_frame((10, 10, 280, 60)).place.all_ver();

        self.fps_label.set_text("fps label");
        self.frame_drawn_label.set_text("frame drawn label");

        if Platform::MOBILE {
            self.set_origin((28, 28));
        } else {
            self.set_origin((10, 10));
        }

        let mut this = self.weak();
        self.fps.on_set.sub(move |fps| {
            this.fps_label.set_text(format!("FPS: {}", fps));
        });

        self.url.on_set.sub(move |url| {
            this.url_label.set_text(url);
        });
    }

    fn update(&mut self) {
        self.frame_drawn += 1;
        self.frame_drawn_label.set_text(format!("Frame drawn: {}", self.frame_drawn));
    }
}
