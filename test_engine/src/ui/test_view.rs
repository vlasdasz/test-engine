use crate::paths;
use gl_image::Image;
use gm::flat::PointsPath;
use gm::Color;
use proc_macro::AsAny;
use proc_macro::New;
use tools::has_new::new;
use tools::refs::{make_shared, new_shared, Shared};
use tools::rglica::ToRglica;
use tools::New;
use tools::Rglica;
use ui::basic::Button;
use ui::complex::{AnalogStickView, DrawingView};
use ui::{DPadView, ImageView, Label, View, ViewBase};

static mut COUNTER: u32 = 0;

#[derive(AsAny, New)]
pub struct TestView {
    base: ViewBase,
    pub data: u128,
    pub clicks: u128,
    pub image: Rglica<ImageView>,
    pub label: Rglica<Label>,
    pub dpad: Rglica<DPadView>,
    pub left_stick: Rglica<AnalogStickView>,
    pub right_stick: Rglica<AnalogStickView>,
}

impl View for TestView {
    fn setup(&mut self) {
        self.set_frame((10, 10, 1000, 500).into());

        // let cat_image = self.image.clone();
        // let mut cat_image = cat_image.borrow_mut();
        // cat_image.image = Image::load(&paths::images().join("cat.jpg"));
        // cat_image.set_frame((200, 20, 100, 120).into());
        // drop(cat_image);
        // self.add_subview(self.image.clone());
        //

        let mut label = Box::new(Label::new());
        label.set_text("ti stragadag stragadag4naja stragadag stragadag stragadakt4ka");
        label.frame_mut().origin.y = 240.0;
        self.label = label.to_rglica();
        self.add_subview(label);

        // let mut view = ViewBase::new();
        // view.set_frame((10, 20, 50, 50).into());
        // view.set_color(Color::WHITE);
        //
        // let mut button = Button::new();
        // button.set_frame((10, 10, 20, 20).into());
        // button.set_color(Color::RED);
        //
        // let label = self.label.clone();
        //
        // button.on_tap.subscribe(move |_| unsafe {
        //     label.borrow_mut().set_text(&format!("kok: {}", COUNTER));
        //     COUNTER += 1;
        // });
        //
        // view.add_subview(make_shared(button));

        //self.add_subview(make_shared(view));

        let mut dpad = Box::new(DPadView::new());
        dpad.frame_mut().size.width = 280.0;
        dpad.frame_mut().size.height = 200.0;
        dpad.frame_mut().origin.y = 300.0;

        self.dpad = dpad.to_rglica();

        self.add_subview(dpad);

        self.dpad.set_images(
            Image::load(&paths::images().join("up.png")),
            Image::load(&paths::images().join("down.png")),
            Image::load(&paths::images().join("left.png")),
            Image::load(&paths::images().join("right.png")),
        );

        self.dpad.on_up.subscribe(|_| {
            dbg!("kkk");
        });

        // let mut drawing = DrawingView::new();
        //
        // drawing.set_frame((500, 10, 200, 200).into());
        //
        // let mut path = PointsPath::new();
        //
        // path.add_point((1, 20).into());
        // path.add_point((100, 30).into());
        // path.add_point((1, 40).into());
        // path.add_point((200, 50).into());
        // path.add_point((1, 60).into());
        // path.add_point((300, 70).into());
        //
        // drawing.add_path(path, Color::GREEN);
        //
        // self.add_subview(make_shared(drawing));
        //

        let left_stick = Box::new(AnalogStickView::new());
        self.left_stick = left_stick.to_rglica();
        self.add_subview(left_stick);
        self.left_stick.frame_mut().origin.x = 320.0;
        self.left_stick.frame_mut().origin.y = 300.0;

        let right_stick = Box::new(AnalogStickView::new());
        self.right_stick = right_stick.to_rglica();
        self.add_subview(right_stick);
        self.right_stick.frame_mut().origin.x = 520.0;
        self.right_stick.frame_mut().origin.y = 300.0;
    }

    fn update(&mut self) {
        self.data += 1;
    }

    fn layout(&mut self) {
        self.view_mut().lay.br();
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
