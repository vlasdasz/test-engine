use crate::ui::view::View;
use crate::ui::{Font, ViewBase};
use std::any::Any;
use tools::{AsAny, HasNew};

#[derive(Debug)]
pub struct DebugView {
    view: ViewBase,
    pub font: Font,
}

impl View for DebugView {

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

impl HasNew for DebugView {
    fn new() -> Self {
        DebugView {
            view: ViewBase::new(),
            font: Font::blank(),
        }
    }
}
