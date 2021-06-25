use crate::gm::{Rect, Size};
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
        self.set_frame(Rect::make(200.0, 200.0, 400.0, 100.0).into());

        self.make_subview(|view| {
            view.set_frame(Rect::make(10.0, 20.0, 50.0, 50.0));

            view.make_subview(|view| {
               view.set_frame(Rect::make(5.0, 5.0, 5.0, 5.0));
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
