use crate::ui::view::View;
use crate::ui::ViewBase;
use std::any::Any;
use tools::{AsAny, HasNew};

#[derive(Debug)]
pub struct DebugView {
    view: ViewBase,
}

impl View for DebugView {
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
    fn new() -> Self
    where
        Self: Sized,
    {
        DebugView {
            view: ViewBase::new(),
        }
    }
}
