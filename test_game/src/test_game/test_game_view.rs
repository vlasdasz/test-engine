use test_engine::{
    audio::Sound,
    game_view::GameView,
    rtools::{
        data_manager::{DataManager, Handle},
        Boxed, Rglica, ToRglica,
    },
    sprite_view::SpriteView,
    sprites::Control,
    ui::{
        basic::Button,
        complex::{AnalogStickView, LabeledSlider},
        placer::Anchor,
        test::test_view::TestView,
        view_base::{add_view, add_view_with_frame, make_view_on, ViewBase},
        DPadView, View,
    },
    ui_layer::UILayer,
    Image, Level,
};

use crate::{test_game::test_game_level::TestGameLevel, BenchmarkView};

#[derive(Default, Debug)]
pub struct TestGameView {
    base:        ViewBase,
    level:       TestGameLevel,
    dpad:        Rglica<DPadView>,
    left_stick:  Rglica<AnalogStickView>,
    sprote_view: Rglica<SpriteView>,
    test_view:   Rglica<TestView>,

    _ui_scale_slider:  Rglica<LabeledSlider>,
    game_scale_slider: Rglica<LabeledSlider>,

    to_benchmark: Rglica<Button>,

    play:  Rglica<Button>,
    sound: Handle<Sound>,

    ui: Rglica<UILayer>,
}

impl TestGameView {
    fn setup_level(&mut self) {
        self.level.setup();

        self.dpad
            .on_press
            .subscribe(self.level.player, move |dir, mut player| {
                player.move_by_direction(dir)
            });

        self.left_stick
            .on_change
            .subscribe(self.level.player, move |dir, mut player| {
                player.add_impulse(dir);
            });
    }

    fn setup_sliders(&mut self) {
        self.game_scale_slider = add_view_with_frame(self, (50, 280));
        self.game_scale_slider.set_multiplier(10.0);

        self.game_scale_slider
            .on_change
            .subscribe(self.to_rglica(), move |value, mut this| {
                this.level_mut().drawer_mut().set_scale(value);
            });
    }

    fn setup_ui(&mut self) {
        self.set_frame((10, 10, 1000, 500).into());

        self.sprote_view = add_view_with_frame(self, (500, 180));

        self.level
            .base()
            .on_sprite_selected
            .subscribe(self.to_rglica(), move |sprite, mut this| {
                this.sprote_view.set_sprite(sprite)
            });

        self.dpad = make_view_on(self, |dpad: &mut DPadView| {
            dpad.frame_mut().size = (200, 150).into();

            dpad.set_images(
                Image::get("up.png"),
                Image::get("down.png"),
                Image::get("left.png"),
                Image::get("right.png"),
            );
        });

        self.left_stick = add_view(self);

        self.setup_sliders();

        self.test_view = add_view_with_frame(self, (280, 400));
        self.test_view.set_image(Image::get("cat.png"));
        self.test_view.set_button_image(Image::get("square.png"));
        self.test_view.set_animation_image(Image::get("palm.png"));

        self.to_benchmark = add_view(self);
        self.to_benchmark.set_text("Benchmark");
        self.to_benchmark.frame_mut().size = (120, 20).into();
        self.to_benchmark
            .on_tap
            .subscribe(self.to_rglica(), move |_, mut this| {
                this.ui.set_view(BenchmarkView::boxed());
            });

        self.play = make_view_on(self, |play: &mut Button| {
            play.set_text("Play sound");
            play.frame_mut().size = (120, 20).into();
        });
        self.play
            .on_tap
            .subscribe(self.to_rglica(), move |_, mut this| this.sound.play());

        self.sound = Sound::get("retro.wav");
    }
}

impl View for TestGameView {
    fn setup(&mut self) {
        self.setup_ui();
        self.setup_level();
    }

    fn layout(&mut self) {
        self.place().as_background();

        self.dpad.place().bottom_left(5);
        self.left_stick
            .place()
            .anchor(self.dpad, Anchor::Right, Anchor::Bot, 20);

        self.game_scale_slider.place().proportional_height(0.5);
        self.game_scale_slider
            .place()
            .anchor(self.dpad, Anchor::Top, Anchor::Left, 10);

        self.sprote_view
            .place()
            .anchor(self.game_scale_slider, Anchor::Right, Anchor::Bot, 10);

        self.test_view.place().bottom_right(20);
        self.test_view.place().proportional_width(0.28);
        self.test_view.place().proportional_height(0.8);

        self.to_benchmark.place().bottom_center(20);

        self.play
            .place()
            .anchor(self.to_benchmark, Anchor::Top, Anchor::Center, 10);
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

    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}
