use rtools::{platform::Platform, Property, Rglica, ToRglica};
use ui::{
    view_base::{add_view, ViewBase},
    Label, View,
};

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
        self.frame_mut().size = (100, 280).into();

        self.fps_label = add_view(self);
        self.fps_label.set_text("fps label");

        self.frame_drawn_label = add_view(self);
        self.frame_drawn_label.set_text("frame drawn label");

        if Platform::MOBILE {
            self.frame_mut().origin.x = 28.0;
            self.frame_mut().origin.y = 28.0;
        }

        let mut this = self.to_rglica();
        self.fps.on_set.subscribe(move |_| {
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
