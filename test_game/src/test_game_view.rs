use std::{borrow::BorrowMut, ops::Deref};

use rtools::{Event, Rglica, ToRglica};
use test_engine::{
    assets::Assets,
    gm::Color,
    sprites::Control,
    ui::{
        basic::{Button, Circle},
        complex::{AnalogStickView, DrawingView, Slider},
        init_view_on, init_view_with_frame, make_view_on, DPadView, ImageView, Label, View,
        ViewBase,
    },
    ui_layer::GameView,
    Level,
};

use crate::test_game_level::TestGameLevel;

static mut COUNTER: u32 = 0;

#[derive(Default)]
pub struct TestGameView {
    base:         ViewBase,
    level:        TestGameLevel,
    image_view:   Rglica<ImageView>,
    label:        Rglica<Label>,
    dpad:         Rglica<DPadView>,
    left_stick:   Rglica<AnalogStickView>,
    circle:       Rglica<Circle>,
    slider:       Rglica<Slider>,
    slider_label: Rglica<Label>,
    set_scale:    Event<f32>,
}

impl TestGameView {
    fn setup_level(&mut self) {
        self.level.setup();

        let mut level = self.level.borrow_mut().to_rglica();

        let mut lvl = level.clone();
        self.dpad
            .on_press
            .subscribe(move |direction| lvl.player().move_by_direction(direction));

        self.left_stick
            .on_direction_change
            .subscribe(move |direction| {
                level.player().add_impulse(&direction);
            });
    }

    fn setup_slider(&mut self) {
        self.slider = init_view_with_frame((50, 280).into(), self);
        self.slider.multiplier = 50.0;

        self.slider_label = init_view_with_frame((50, 50).into(), self);
        self.slider_label.set_text("hello");

        let mut this = self.to_rglica();
        self.slider.on_change.subscribe(move |value| {
            this.slider_label.set_text(value.to_string());
            this.set_scale.trigger(value);
        });
    }

    fn setup_ui(&mut self) {
        self.set_frame((10, 10, 1000, 500).into());

        self.image_view = make_view_on(self, |view: &mut ImageView| {
            view.image = Assets::image("cat.png");
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
                Assets::image("up.png"),
                Assets::image("down.png"),
                Assets::image("left.png"),
                Assets::image("right.png"),
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
    }
}

impl View for TestGameView {
    fn setup(&mut self) {
        self.setup_ui();
        self.setup_level();
    }

    fn layout(&mut self) {
        self.place().bottom_right();
        self.frame_mut().size.width = self.super_frame().width();
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

impl GameView for TestGameView {
    fn level(&self) -> &dyn Level {
        &self.level
    }
    fn level_mut(&mut self) -> &mut dyn Level {
        &mut self.level
    }
}
