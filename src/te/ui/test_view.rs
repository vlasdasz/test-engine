use crate::gm::{Color, Rect};
use crate::image::Image;
use crate::ui::view::View;
use crate::ui::{Font, ImageView, Label, Layout, ViewBase};
use std::any::Any;
use tools::refs::{make_shared, Shared};
use tools::{AsAny, HasNew};
use crate::ui::basic::Button;

#[derive(Debug)]
pub struct TestView {
    base: ViewBase,
    pub data: u128,
    pub font: Font,
    pub image: Option<Shared<ImageView>>,
    pub label: Option<Shared<Label>>,
}

impl View for TestView {
    fn setup(&mut self, _: Shared<dyn View>) {
        self.set_frame(Rect::make(10, 10, 680, 500));

        let mut cat_image = ImageView::new();
        cat_image.image = Image::load(&crate::te::paths::images().join("cat.jpg"));
        cat_image.set_frame(Rect::make(200, 20, 100, 120));
        let shared_cat = make_shared(cat_image);
        self.image = Some(shared_cat.clone());
        self.add_subview(shared_cat);

        self.make_subview(|view| {
            view.set_color(Color::WHITE);
            view.set_frame(Rect::make(10, 20, 50, 50));

            let mut button = Button::new();
            button.set_frame(Rect::make(10, 10, 20, 20));
            button.set_color(Color::RED);

            button.on_tap.subscribe(|_| {
               dbg!("Hellof");
            });

            view.add_subview(make_shared(button));

        });

        let mut label = Label::from_rect(Rect::make(5, 200, 100, 100));
        label.set_text("ti stragadag stragadag4naja stragadag stragadag stragadakt4ka");
        label.font = self.font.clone();
        let shared_label = make_shared(label);
        self.label = Some(shared_label.clone());
        self.add_subview(shared_label);
    }

    fn update(&mut self) {
        guard!(let Some(label) = &self.label else {
           return;
        });

        let mut label = label.try_borrow_mut().unwrap();

        label.set_text(&format!(
            "ti stragadag stragadag4naja stragadag stragadag stragadakt4ka: {}",
            self.data
        ));

        self.data += 1;
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }

    fn layout(&mut self, _super_frame: &Rect) {
        Layout::br(self.frame_mut(), _super_frame);
    }
}

impl HasNew for TestView {
    fn new() -> Self {
        TestView {
            base: ViewBase::new(),
            data: 0,
            font: Font::blank(),
            image: None,
            label: None,
        }
    }
}

impl AsAny for TestView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
