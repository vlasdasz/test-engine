use crate::gm::Rect;
use crate::image::Image;
use crate::ui::view::View;
use crate::ui::{Font, ImageView, Label, Layout, ViewBase};
use std::any::Any;
use tools::refs::make_shared;
use tools::{AsAny, HasNew};

#[derive(Debug)]
pub struct TestView {
    base: ViewBase,
    pub font: Font,
}

impl View for TestView {
    fn setup(&mut self) {
        self.set_frame(Rect::make(10, 10, 680, 400).into());

        self.make_subview(|view| {
            view.set_frame(Rect::make(10, 20, 50, 50));
            view.enable_touch();

            view.make_subview(|view| {
                view.set_frame(Rect::make(5, 5, 5, 5));
            });

            let mut cat_image = ImageView::new();
            cat_image.image = Image::load(&crate::te::paths::images().join("cat.jpg"));
            cat_image.set_frame(Rect::make(200, 20, 100, 120));
            view.add_subview(make_shared(cat_image));
        });

        let mut label = Label::from_rect(Rect::make(40, 200, 100, 100));
        label.set_text("ti stragadag stragadag4naja stragadag stragadag stragadakt4ka");
        label.font = self.font.clone();
        self.add_subview(make_shared(label));
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

    fn layout(&mut self, _super_frame: &Rect) {
        Layout::br(self.frame_mut(), _super_frame);
    }
}

impl HasNew for TestView {
    fn new() -> Self {
        TestView {
            base: ViewBase::new(),
            font: Font::blank(),
        }
    }
}

impl AsAny for TestView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
