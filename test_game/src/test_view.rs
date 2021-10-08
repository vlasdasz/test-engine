use std::ops::Deref;

use test_engine::{
    gm::Color,
    screen::GameView,
    sprites::Control,
    ui::{
        basic::{Button, Circle},
        complex::{AnalogStickView, DrawingView, Slider},
        init_view_on, init_view_with_frame, make_view_on, DPadView, ImageView, Label, View,
        ViewBase,
    },
    Image, Level,
};
use tools::Rglica;

use crate::test_level::TestLevel;

static mut COUNTER: u32 = 0;

#[derive(Default)]
pub struct TestView {
    base:         ViewBase,
    level:        TestLevel,
    data:         u128,
    image_view:   Rglica<ImageView>,
    label:        Rglica<Label>,
    dpad:         Rglica<DPadView>,
    left_stick:   Rglica<AnalogStickView>,
    circle:       Rglica<Circle>,
    slider:       Rglica<Slider>,
    slider_label: Rglica<Label>,
}

impl TestView {
    fn setup_level(&mut self) {
        let mut level = Rglica::from_ref(&self.level);

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

impl TestView {
    fn setup_slider(&mut self) {
        self.slider = init_view_with_frame((50, 280).into(), self);
        self.slider_label = init_view_with_frame((50, 50).into(), self);
        self.slider_label.set_text("hello");

        let mut label = self.slider_label.clone();
        self.slider.on_change.subscribe(move |value| {
            label.set_text(value.to_string())
            //dbg!(value);
        });
    }
}

impl View for TestView {
    fn setup(&mut self) {
        self.set_frame((10, 10, 1000, 500).into());

        self.image_view = make_view_on(self, |view: &mut ImageView| {
            view.image = Image::load(&test_engine::paths::images().join("cat.png"));
            view.set_frame((200, 20, 100, 120).into());
        });

        self.label = make_view_on(self, |view: &mut Label| {
            view.set_text("ti stragadag stragadag4naja stragadag stragadag stragadakt4ka");
            view.frame_mut().origin.y = 240.0;
        });

        let mut label = self.label.clone();
        make_view_on(self, |view: &mut ViewBase| {
            view.set_frame((10, 20, 50, 50).into());
            view.set_color(Color::WHITE);

            make_view_on(view, |button: &mut Button| {
                button.set_frame((10, 10, 20, 20).into());
                button.set_color(Color::RED);
                button.on_tap.subscribe(move |_| unsafe {
                    label.set_text(&format!("kok: {}", COUNTER));
                    COUNTER += 1;
                });
            });
        });

        self.dpad = make_view_on(self, |dpad: &mut DPadView| {
            dpad.frame_mut().size = (280, 200).into();
            dpad.frame_mut().origin.y = 300.0;

            dpad.set_images(
                Image::load(&test_engine::paths::images().join("up.png")),
                Image::load(&test_engine::paths::images().join("down.png")),
                Image::load(&test_engine::paths::images().join("left.png")),
                Image::load(&test_engine::paths::images().join("right.png")),
            );
        });

        make_view_on(self, |drawing: &mut DrawingView| {
            drawing.set_frame((500, 10, 200, 200).into());
            drawing.add_path(
                vec![
                    (1, 20).into(),
                    (100, 30).into(),
                    (1, 40).into(),
                    (200, 50).into(),
                    (1, 60).into(),
                    (1, 20).into(),
                    (300, 70).into(),
                ]
                .into(),
                Color::GREEN,
            );
        });

        self.left_stick = init_view_on(self);
        self.left_stick.frame_mut().origin = (320, 300).into();

        self.circle = init_view_with_frame((50, 50).into(), self);
        self.circle.set_color(Color::GREEN);

        self.setup_slider();

        self.setup_level();
    }

    fn update(&mut self) {
        self.data += 1
    }

    fn layout(&mut self) {
        self.place().bottom_right();
        self.frame_mut().size.width = self.super_frame().size.width;
        self.circle.place().bottom_right_margin(20);
        self.slider.place().top_right_margin(20);
        self.slider_label.place().at_bottom(self.slider.deref(), 20);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl GameView for TestView {
    fn level(&self) -> &dyn Level {
        &self.level
    }
    fn level_mut(&mut self) -> &mut dyn Level {
        &mut self.level
    }
}
