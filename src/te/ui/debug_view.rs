use crate::gm::Rect;
use crate::ui::view::View;
use crate::ui::{Font, Label, ViewBase};
use std::any::Any;
use tools::refs::make_box;
use tools::{AsAny, HasNew};

#[derive(Debug)]
pub struct DebugView {
    view: ViewBase,
    pub font: Font,
}

impl DebugView {}

impl View for DebugView {
    fn setup(&mut self) {
        self.set_frame(Rect::make(200, 200, 680, 400).into());

        self.make_subview(|view| {
            view.set_frame(Rect::make(10, 20, 50, 50));

            view.make_subview(|view| {
                view.set_frame(Rect::make(5, 5, 5, 5));
            });
        });

        let mut label = Label::from_rect(Rect::make(40, 200, 100, 100));
        label.set_text("ti stragadag stragadag4naja stragadag stragadag stragadakt4ka");
        label.font = self.font.clone();
        self.add_subview(make_box(label));
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
            font: Font::blank(),
        }
    }
}
