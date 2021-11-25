use std::default::default;

use tools::{platform::Platform, Boxed, Property, Rglica, ToRglica};
use ui::{init_view_on, Label, View, ViewBase};

pub struct DebugView {
    view:              ViewBase,
    fps_label:         Rglica<Label>,
    frame_drawn_label: Rglica<Label>,
    frame_drawn:       u64,
    pub fps:           Property<u64>,
}

impl View for DebugView {
    fn setup(&mut self) {
        self.frame_mut().size.height = 100.0;
        self.frame_mut().size.width = 280.0;

        self.fps_label = init_view_on(self);
        self.fps_label.set_text("fps label");

        self.frame_drawn_label = init_view_on(self);
        self.frame_drawn_label.set_text("frame drawn label");

        if Platform::MOBILE {
            self.frame_mut().origin.x = 28.0;
            self.frame_mut().origin.y = 28.0;
        }

        dbg!("sinka");
        let mut this = self.to_rglica();
        self.fps.on_set.subscribe(move |_| {
            let fps = this.fps.copy();
            dbg!(fps);
            this.fps_label.set_text(format!("FPS: {}", fps));
        });
    }

    fn update(&mut self) {
        self.frame_drawn += 1;
        self.frame_drawn_label
            .set_text(format!("Frame drawn: {}", self.frame_drawn));
    }

    fn layout(&mut self) {
        self.place().top_left_margin(10);
        self.place().subviews_vertically();
    }

    fn view(&self) -> &ViewBase {
        &self.view
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }
}

impl Boxed for DebugView {
    fn boxed() -> Box<Self> {
        dbg!("krito");
        Box::new(DebugView {
            view:              default(),
            fps_label:         default(),
            frame_drawn_label: default(),
            frame_drawn:       0,
            fps:               Default::default(),
        })
    }
}
