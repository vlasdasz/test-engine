use rtools::{platform::Platform, Property, Rglica};
use ui::{Label, View, ViewBase, ViewFrame, ViewTemplates};

#[derive(Default, Debug)]
pub struct DebugView {
    view:              ViewBase,
    fps_label:         Rglica<Label>,
    frame_drawn_label: Rglica<Label>,
    frame_drawn:       u64,
    pub fps:           Property<u64>,
}

impl View for DebugView {
    fn setup(&mut self) {
        self.set_frame((280, 100));

        self.fps_label = self.add_view();
        self.fps_label.set_text("fps label");

        self.frame_drawn_label = self.add_view();
        self.frame_drawn_label.set_text("frame drawn label");

        if Platform::MOBILE {
            self.set_origin((28, 28));
        }

        self.fps.on_set.set(self, |_, this| {
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

    fn view(&self) -> &ViewBase {
        &self.view
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }
}
