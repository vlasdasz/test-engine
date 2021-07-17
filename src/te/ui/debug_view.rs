use crate::gm::Rect;
use crate::ui::view::View;
use crate::ui::{Label, Layout, ViewBase};
use std::any::Any;
use tools::has_new::new;
use tools::refs::{new_shared, Shared};
use tools::{AsAny, New};

#[derive(Debug)]
pub struct DebugView {
    view: ViewBase,
    fps_label: Shared<Label>,
    frame_drawn_label: Shared<Label>,
    frame_drawn: u64,
}

impl View for DebugView {
    fn setup(&mut self, _: Shared<dyn View>) {
        self.frame_mut().size.height = 200.0;
        self.frame_mut().size.width = 400.0;

        self.add_subview(self.fps_label.clone());
        self.add_subview(self.frame_drawn_label.clone());

        self.fps_label.borrow_mut().set_text("fps label");
        self.frame_drawn_label
            .borrow_mut()
            .set_text("frame drawn label");
    }

    fn update(&mut self) {
        self.frame_drawn += 1;
        self.frame_drawn_label
            .borrow_mut()
            .set_text(&format!("Frame drawn: {}", self.frame_drawn));
    }

    fn layout(&mut self, _super_frame: &Rect) {
        Layout::distribute_vertically(&self.frame().clone(), self.subviews_mut());
    }

    fn view(&self) -> &ViewBase {
        &self.view
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }
}

impl AsAny for DebugView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl New for DebugView {
    fn new() -> Self {
        DebugView {
            view: new(),
            fps_label: new_shared(),
            frame_drawn_label: new_shared(),
            frame_drawn: 0,
        }
    }
}
