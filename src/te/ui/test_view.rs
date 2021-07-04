use crate::ui::ViewBase;
use crate::ui::view::View;
use tools::{HasNew, AsAny};
use std::any::Any;

#[derive(Debug)]
struct TestView {
    base: ViewBase
}

impl View for TestView {
    fn setup(&mut self) {

    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }

    fn ptr(&self) -> *const dyn View {
        self as *const dyn View
    }
}

impl HasNew for TestView {
    fn new() -> Self {
        TestView {
            base: ViewBase::new()
        }
    }
}

impl AsAny for TestView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}