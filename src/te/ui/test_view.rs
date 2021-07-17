use crate::gm::{Color, Rect};
use crate::image::Image;
use crate::ui::basic::Button;
use crate::ui::view::View;
use crate::ui::{DPadView, ImageView, Label, Layout, ViewBase};
use std::any::Any;
use tools::has_new::new;
use tools::refs::{make_shared, new_shared, Shared};
use tools::{AsAny, New};

static mut COUNTER: u32 = 0;

#[derive(Debug)]
pub struct TestView {
    base: ViewBase,
    pub data: u128,
    pub clicks: u128,
    pub image: Shared<ImageView>,
    pub label: Shared<Label>,
    pub dpad: Shared<DPadView>,
}

impl View for TestView {
    fn setup(&mut self, _this: Shared<dyn View>) {
        self.set_frame(Rect::make(10, 10, 1000, 500));

        let cat_image = self.image.clone();
        let mut cat_image = cat_image.borrow_mut();
        cat_image.image = Image::load(&crate::te::paths::images().join("cat.jpg"));
        cat_image.set_frame(Rect::make(200, 20, 100, 120));
        drop(cat_image);
        self.add_subview(self.image.clone());

        let label = self.label.clone();
        let mut label = label.borrow_mut();
        label.set_text("ti stragadag stragadag4naja stragadag stragadag stragadakt4ka");
        label.frame_mut().origin.y = 240.0;
        drop(label);
        self.add_subview(self.label.clone());

        let mut view = ViewBase::new();
        view.set_frame(Rect::make(10, 20, 50, 50));
        view.set_color(Color::WHITE);

        let mut button = Button::new();
        button.set_frame(Rect::make(10, 10, 20, 20));
        button.set_color(Color::RED);

        let label = self.label.clone();

        button.on_tap.subscribe(move |_| unsafe {
            label.borrow_mut().set_text(&format!("kok: {}", COUNTER));
            COUNTER += 1;
        });

        view.add_subview(make_shared(button));

        self.add_subview(make_shared(view));

        let shared_dpad = self.dpad.clone();
        let mut dpad = shared_dpad.borrow_mut();
        dpad.frame_mut().size.width = 280.0;
        dpad.frame_mut().size.height = 200.0;
        dpad.frame_mut().origin.y = 300.0;

        dpad.on_up.subscribe(|_| {
            dbg!("up");
        });

        dpad.on_down.subscribe(|_| {
            dbg!("down");
        });

        dpad.on_left.subscribe(|_| {
            dbg!("left");
        });

        dpad.on_right.subscribe(|_| {
            dbg!("right");
        });

        drop(dpad);

        self.add_subview(shared_dpad);
    }

    fn update(&mut self) {
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

impl New for TestView {
    fn new() -> Self {
        TestView {
            base: new(),
            data: 0,
            clicks: 0,
            image: new_shared(),
            label: new_shared(),
            dpad: new_shared(),
        }
    }
}

impl AsAny for TestView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
