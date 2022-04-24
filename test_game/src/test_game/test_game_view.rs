use test_engine::{
    audio::Sound,
    game_view::GameView,
    rtools::{
        data_manager::{DataManager, Handle},
        Boxed, Rglica,
    },
    sprite_view::SpriteView,
    sprites::Control,
    ui::{
        basic::Button,
        complex::{AnalogStickView, LabeledSlider},
        placer::Anchor,
        test::test_view::TestView,
        view_base::ViewBase,
        DPadView, View, ViewTemplates,
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
    sprite_view: Rglica<SpriteView>,
    test_view:   Rglica<TestView>,

    ui_scale_slider:   Rglica<LabeledSlider>,
    game_scale_slider: Rglica<LabeledSlider>,

    to_benchmark: Rglica<Button>,

    play:  Rglica<Button>,
    sound: Handle<Sound>,

    ui: Rglica<UILayer>,
}

impl TestGameView {
    fn setup_level(&mut self) {
        self.level.setup();

        self.dpad.on_press.set(&self.level.player, move |dir, player| {
            player.move_by_direction(dir)
        });

        self.left_stick
            .on_change
            .set(&self.level.player, move |dir, player| {
                player.add_impulse(dir);
            });
    }

    fn setup_sliders(&mut self) {
        self.game_scale_slider = self.add_view_with_frame((50, 280));
        self.game_scale_slider.set_start(0.5).set_finish(10);
        self.game_scale_slider.on_change.set(self, move |scale, this| {
            this.level_mut().drawer_mut().set_scale(scale);
        });

        self.ui_scale_slider = self.add_view_with_frame((50, 280));
        self.ui_scale_slider.set_start(0.2).set_finish(4);
        self.ui_scale_slider.on_change.set(self, move |scale, this| {
            this.ui.set_scale(scale);
        });
    }

    fn setup_ui(&mut self) {
        self.set_frame((10, 10, 1000, 500));

        self.sprite_view = self.add_view_with_frame((500, 180));

        self.level
            .base()
            .on_sprite_selected
            .set(self, move |sprite, this| this.sprite_view.set_sprite(sprite));

        self.dpad = self.add_view();
        self.dpad.set_frame((200, 150)).set_images(
            Image::get("up.png"),
            Image::get("down.png"),
            Image::get("left.png"),
            Image::get("right.png"),
        );

        self.left_stick = self.add_view();

        self.setup_sliders();

        self.test_view = self.add_view_with_frame((280, 400));
        self.test_view
            .set_image(Image::get("cat.png"))
            .set_button_image(Image::get("square.png"))
            .set_animation_image(Image::get("palm.png"));

        self.to_benchmark = self.add_view();
        self.to_benchmark.set_text("Benchmark").set_frame((120, 20));
        self.to_benchmark.on_tap.set(self, move |_, this| {
            this.ui.set_view(BenchmarkView::boxed());
        });

        self.play = self.add_view();
        self.play.set_text("Play sound").set_frame((120, 20));
        self.play.on_tap.set(self, move |_, this| this.sound.play());

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

        self.game_scale_slider.place().proportional_height(0.5).anchor(
            self.dpad,
            Anchor::Top,
            Anchor::Left,
            10,
        );

        self.ui_scale_slider.place().proportional_height(0.5).anchor(
            self.game_scale_slider,
            Anchor::Right,
            Anchor::Center,
            10,
        );

        self.sprite_view
            .place()
            .anchor(self.ui_scale_slider, Anchor::Right, Anchor::Bot, 10);

        self.test_view
            .place()
            .bottom_right(20)
            .proportional_width(0.28)
            .proportional_height(0.8);

        self.to_benchmark.place().bottom_center(20);

        self.play
            .place()
            .anchor(self.to_benchmark, Anchor::Top, Anchor::Center, 10);
    }

    fn update(&mut self) {
        // let pos = self.ui.ui_cursor_position;
        // add_view_with_frame::<ViewBase>(self, (pos.x, pos.y, 5, 5));
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
