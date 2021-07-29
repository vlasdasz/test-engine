use crate::paths;
use gl_image::Image;
use gm::flat::PointsPath;
use gm::{Color, Rect};
use std::any::Any;
use tools::has_new::new;
use tools::refs::{make_shared, new_shared, Shared};
use tools::{AsAny, New};
use ui::basic::Button;
use ui::complex::{AnalogStickView, DrawingView};
use ui::{DPadView, ImageView, Label, Layout, View, ViewBase};

static mut COUNTER: u32 = 0;

#[derive(Debug)]
pub struct TestView {
    base: ViewBase,
    pub data: u128,
    pub clicks: u128,
    pub image: Shared<ImageView>,
    pub label: Shared<Label>,
    pub dpad: Shared<DPadView>,
    pub left_stick: Shared<AnalogStickView>,
    pub right_stick: Shared<AnalogStickView>,
}

impl View for TestView {
    fn setup(&mut self, _this: Shared<dyn View>) {
        self.set_frame((10, 10, 1000, 500).into());

        let cat_image = self.image.clone();
        let mut cat_image = cat_image.borrow_mut();
        cat_image.image = Image::load(&paths::images().join("cat.jpg"));
        cat_image.set_frame((200, 20, 100, 120).into());
        drop(cat_image);
        self.add_subview(self.image.clone());

        let label = self.label.clone();
        let mut label = label.borrow_mut();
        label.set_text("ti stragadag stragadag4naja stragadag stragadag stragadakt4ka");
        label.frame_mut().origin.y = 240.0;
        drop(label);
        self.add_subview(self.label.clone());

        let mut view = ViewBase::new();
        view.set_frame((10, 20, 50, 50).into());
        view.set_color(Color::WHITE);

        let mut button = Button::new();
        button.set_frame((10, 10, 20, 20).into());
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

        dpad.set_images(
            Image::load(&paths::images().join("up.png")),
            Image::load(&paths::images().join("down.png")),
            Image::load(&paths::images().join("left.png")),
            Image::load(&paths::images().join("right.png")),
        );

        drop(dpad);

        self.add_subview(shared_dpad);

        let mut drawing = DrawingView::new();

        drawing.set_frame((500, 10, 200, 200).into());

        let mut path = PointsPath::new();

        path.add_point((1, 20).into());
        path.add_point((100, 30).into());
        path.add_point((1, 40).into());
        path.add_point((200, 50).into());
        path.add_point((1, 60).into());
        path.add_point((300, 70).into());

        drawing.add_path(path, Color::GREEN);

        self.add_subview(make_shared(drawing));

        self.add_subview(self.left_stick.clone());
        self.left_stick.borrow_mut().frame_mut().origin.x = 320.0;
        self.left_stick.borrow_mut().frame_mut().origin.y = 300.0;

        self.add_subview(self.right_stick.clone());
        self.right_stick.borrow_mut().frame_mut().origin.x = 520.0;
        self.right_stick.borrow_mut().frame_mut().origin.y = 300.0;
    }

    fn update(&mut self) {
        self.data += 1;
    }

    fn layout(&mut self, _super_frame: &Rect) {
        Layout::br(self.frame_mut(), _super_frame);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
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
            left_stick: new_shared(),
            right_stick: new_shared(),
        }
    }
}

impl AsAny for TestView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
