use crate::gm::Rect;
use crate::ui::complex::IntView;
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
    scale_view: Shared<IntView>,
}

impl View for DebugView {
    fn setup(&mut self, _: Shared<dyn View>) {
        self.frame_mut().size.height = 100.0;
        self.frame_mut().size.width = 280.0;

        self.add_subview(self.fps_label.clone());
        self.add_subview(self.frame_drawn_label.clone());

        self.fps_label.borrow_mut().set_text("fps label");
        self.frame_drawn_label
            .borrow_mut()
            .set_text("frame drawn label");

        self.add_subview(self.scale_view.clone());

        self.scale_view.borrow_mut().on_change.subscribe(|val| {
            dbg!(val);
        });
    }

    fn update(&mut self) {
        self.frame_drawn += 1;
        self.frame_drawn_label
            .borrow_mut()
            .set_text(&format!("Frame drawn: {}", self.frame_drawn));
    }

    fn layout(&mut self, _super_frame: &Rect) {
        Layout::distribute_vertically(&self.frame().clone(), self.subviews_mut());
        self.scale_view.borrow_mut().frame_mut().size.width = self.frame().width() / 10.0
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
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
            scale_view: new_shared(),
        }
    }
}
