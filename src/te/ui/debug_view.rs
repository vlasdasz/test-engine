use crate::gm::Rect;
use crate::ui::view::View;
use crate::ui::ViewBase;
use std::any::Any;
use tools::{AsAny, HasNew};

#[derive(Debug)]
pub struct DebugView {
    view: ViewBase,
}

impl DebugView {}

impl View for DebugView {
    fn setup(&mut self) {
        self.set_frame(Rect::make(200, 200, 400, 100).into());

        self.make_subview(|view| {
            view.set_frame(Rect::make(10, 20, 50, 50));

            view.make_subview(|view| {
                view.set_frame(Rect::make(5, 5, 5, 5));
            });
        });

        dbg!("hello");
    }

    fn view(&self) -> &ViewBase {
        &self.view
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }

    fn ptr(&self) -> *const dyn View {
        self as *const dyn View
    }
}

impl AsAny for DebugView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HasNew for DebugView {
    fn new() -> Self {
        DebugView {
            view: ViewBase::new(),
        }
    }
}
