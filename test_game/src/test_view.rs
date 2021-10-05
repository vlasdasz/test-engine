use std::ops::{Deref, DerefMut};

use proc_macro::AsAny;
use test_engine::{
    gm::{flat::PointsPath, Color},
    screen::GameView,
    sprites::Control,
    ui::{
        basic::Button,
        complex::{AnalogStickView, DrawingView},
        make_view_on, DPadView, ImageView, Label, View, ViewBase,
    },
    Image, Level,
};
use tools::{new, rglica::ToRglica, Boxed, New, Rglica};

use crate::test_level::TestLevel;

static mut COUNTER: u32 = 0;

#[derive(AsAny)]
pub struct TestView {
    base:            ViewBase,
    level:           Box<TestLevel>,
    pub data:        u128,
    pub clicks:      u128,
    pub image_view:  Rglica<ImageView>,
    pub label:       Rglica<Label>,
    pub dpad:        Rglica<DPadView>,
    pub left_stick:  Rglica<AnalogStickView>,
    pub right_stick: Rglica<AnalogStickView>,
}

impl TestView {
    fn setup_level(&mut self) {
        self.level.setup();

        let mut level = self.level.to_rglica();

        let mut lvl = level.clone();
        self.dpad.on_up.subscribe(move |_| {
            lvl.player().jump();
        });

        let mut lvl = level.clone();
        self.dpad.on_left.subscribe(move |_| {
            lvl.player().go_left();
        });

        let mut lvl = level.clone();
        self.dpad.on_right.subscribe(move |_| {
            lvl.player().go_right();
        });

        self.left_stick
            .on_direction_change
            .subscribe(move |direction| {
                level.player().add_impulse(&direction);
            });
    }
}

impl View for TestView {
    fn setup(&mut self) {
        self.set_frame((10, 10, 1000, 500).into());

        let image_view = ImageView::boxed();
        self.image_view = image_view.to_rglica();
        self.image_view.image = Image::load(&test_engine::paths::images().join("cat.png"));
        self.image_view.set_frame((200, 20, 100, 120).into());
        self.add_subview(image_view);

        self.label = make_view_on(self);

        self.label
            .set_text("ti stragadag stragadag4naja stragadag stragadag stragadakt4ka");
        self.label.frame_mut().origin.y = 240.0;

        let mut view = make_view_on::<ViewBase>(self);

        view.set_frame((10, 20, 50, 50).into());
        view.set_color(Color::WHITE);

        let mut button = make_view_on::<Button>(view.deref_mut());

        button.set_frame((10, 10, 20, 20).into());
        button.set_color(Color::RED);

        let mut label = self.label.clone();
        button.on_tap.subscribe(move |_| unsafe {
            label.set_text(&format!("kok: {}", COUNTER));
            COUNTER += 1;
        });

        self.dpad = make_view_on(self);

        self.dpad.frame_mut().size.width = 280.0;
        self.dpad.frame_mut().size.height = 200.0;
        self.dpad.frame_mut().origin.y = 300.0;

        self.dpad.set_images(
            Image::load(&test_engine::paths::images().join("up.png")),
            Image::load(&test_engine::paths::images().join("down.png")),
            Image::load(&test_engine::paths::images().join("left.png")),
            Image::load(&test_engine::paths::images().join("right.png")),
        );

        let mut drawing = make_view_on::<DrawingView>(self);

        drawing.set_frame((500, 10, 200, 200).into());

        let mut path = PointsPath::new();

        path.add_point((1, 20).into());
        path.add_point((100, 30).into());
        path.add_point((1, 40).into());
        path.add_point((200, 50).into());
        path.add_point((1, 60).into());
        path.add_point((300, 70).into());

        drawing.add_path(path, Color::GREEN);

        let left_stick = AnalogStickView::boxed();
        self.left_stick = left_stick.to_rglica();
        self.add_subview(left_stick);
        self.left_stick.frame_mut().origin.x = 320.0;
        self.left_stick.frame_mut().origin.y = 300.0;

        let right_stick = AnalogStickView::boxed();
        self.right_stick = right_stick.to_rglica();
        self.add_subview(right_stick);
        self.right_stick.frame_mut().origin.x = 520.0;
        self.right_stick.frame_mut().origin.y = 300.0;

        self.setup_level();
    }

    fn update(&mut self) { self.data += 1 }

    fn layout(&mut self) { self.placer().br() }

    fn view(&self) -> &ViewBase { &self.base }

    fn view_mut(&mut self) -> &mut ViewBase { &mut self.base }
}

impl GameView for TestView {
    fn level(&self) -> &dyn Level { self.level.deref() }
    fn level_mut(&mut self) -> &mut dyn Level { self.level.deref_mut() }
}

impl Boxed for TestView {
    fn boxed() -> Box<Self> {
        Box::new(Self {
            base:        ViewBase::default(),
            level:       TestLevel::boxed(),
            data:        0,
            clicks:      0,
            image_view:  new(),
            label:       new(),
            dpad:        new(),
            left_stick:  new(),
            right_stick: new(),
        })
    }
}
