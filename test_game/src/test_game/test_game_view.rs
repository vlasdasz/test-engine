use rtools::{Boxed, Rglica, ToRglica};
use test_engine::{
    assets::Assets,
    game_view::GameView,
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
    Level,
};

use crate::{test_game::test_game_level::TestGameLevel, BenchmarkView};

#[derive(Default, Debug)]
pub struct TestGameView {
    base:       ViewBase,
    level:      TestGameLevel,
    dpad:       Rglica<DPadView>,
    left_stick: Rglica<AnalogStickView>,
    sprite:     Rglica<SpriteView>,
    slider:     Rglica<LabeledSlider>,
    test_view:  Rglica<TestView>,

    to_benchmark: Rglica<Button>,

    ui: Rglica<UILayer>,
}

impl TestGameView {
    fn setup_level(&mut self) {
        self.level.setup();

        let mut player = self.level.player;
        self.dpad
            .on_press
            .subscribe(move |direction| player.move_by_direction(direction));

        self.left_stick
            .on_direction_change
            .subscribe(move |direction| {
                player.add_impulse(direction);
            });
    }

    fn setup_slider(&mut self) {
        self.slider = add_view_with_frame(self, (50, 280));
        self.slider.set_multiplier(10.0);

        let mut this = self.to_rglica();
        self.slider.on_change.subscribe(move |value| {
            this.level_mut().drawer_mut().set_scale(value);
        });
    }

    fn setup_ui(&mut self) {
        self.set_frame((10, 10, 1000, 500).into());

        self.sprite = add_view_with_frame(self, (500, 180));

        let mut this = self.to_rglica();
        self.level
            .base_mut()
            .on_sprite_selected
            .subscribe(move |sprite| this.sprite.set_sprite(sprite));

        self.dpad = make_view_on(self, |dpad: &mut DPadView| {
            dpad.frame_mut().size = (200, 150).into();

            dpad.set_images(
                Assets::image("up.png"),
                Assets::image("down.png"),
                Assets::image("left.png"),
                Assets::image("right.png"),
            );
        });

        self.left_stick = add_view(self);

        self.setup_slider();

        self.test_view = add_view_with_frame(self, (280, 400));
        self.test_view.set_image(Assets::image("cat.png"));
        self.test_view.set_button_image(Assets::image("square.png"));
        self.test_view
            .set_animation_image(Assets::image("palm.png"));

        self.to_benchmark = add_view(self);
        self.to_benchmark.set_text("Benchmark");
        self.to_benchmark.frame_mut().size = (120, 20).into();
        let mut this = self.to_rglica();
        self.to_benchmark.on_tap.subscribe(move |_| {
            this.ui.set_view(BenchmarkView::boxed());
        });
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

        self.slider.place().proportional_height(0.5);
        self.slider
            .place()
            .anchor(self.dpad, Anchor::Top, Anchor::Left, 10);

        self.sprite
            .place()
            .anchor(self.slider, Anchor::Right, Anchor::Bot, 10);

        self.test_view.place().bottom_right(20);
        self.test_view.place().proportional_width(0.28);
        self.test_view.place().proportional_height(0.8);

        self.to_benchmark.place().bottom_center(10);
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
