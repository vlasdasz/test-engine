use serde::{Deserialize, Serialize};
use tao_log::infov;
use test_engine::{
    audio::Sound,
    gm::Color,
    main_view::{HasLevel, MainView},
    net::{GetRequest, API},
    rtools::{
        data_manager::{DataManager, Handle},
        Apply, Rglica, ToRglica,
    },
    sprite_view::SpriteView,
    sprites::{Control, Player},
    ui::{
        basic::Button, complex::AnalogStickView, test::test_view::TestView, view, BaseView, DPadView, View,
        ViewBase, ViewCallbacks, ViewData, ViewFrame, ViewLayout, ViewSubviews,
    },
    ui_layer::UILayer,
    Image, Level,
};

use crate::{test_game::test_game_level::TestGameLevel, BenchmarkView, UITestView};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login:    String,
    password: String,
}

const API: API = API::new("ec2-18-217-89-172.us-east-2.compute.amazonaws.com");

const GET_USERS: GetRequest<Vec<User>> = API.get("get_users");

#[view]
#[derive(Default)]
pub struct TestGameView {
    level:       TestGameLevel,
    dpad:        Rglica<DPadView>,
    left_stick:  Rglica<AnalogStickView>,
    sprite_view: Rglica<SpriteView>,
    test_view:   Rglica<TestView>,

    to_benchmark: Rglica<Button>,
    to_test:      Rglica<Button>,

    play:       Rglica<Button>,
    sound:      Handle<Sound>,
    async_task: Rglica<Button>,

    ui: Rglica<UILayer>,
}

impl TestGameView {
    fn setup_level(&mut self) {
        self.level.setup();

        self.dpad
            .on_press
            .set(&self.level.player, |player, dir| player.move_by_direction(dir));
    }

    fn setup_ui(&mut self) {
        self.ui.keymap.add("=", self, |this| {
            let scale = this.ui.scale() * 1.2;
            this.ui.set_scale(scale);
        });

        self.ui.keymap.add("-", self, |this| {
            let scale = this.ui.scale() * 0.8;
            this.ui.set_scale(scale);
        });

        self.set_frame((10, 10, 1000, 500));

        self.sprite_view = self.add_view();
        self.sprite_view.place().tr().val(10).size(400, 80);

        self.level
            .base()
            .on_sprite_selected
            .set(self, |this, sprite| this.sprite_view.set_sprite(sprite));

        self.dpad = self.add_view();
        self.dpad
            .set_images(
                Image::get("up.png"),
                Image::get("down.png"),
                Image::get("left.png"),
                Image::get("right.png"),
            )
            .place()
            .size(140, 100)
            .bottom()
            .val(10)
            .left()
            .val(100);

        self.left_stick = self.add_view();
        self.left_stick.place().bl().val(10).size(80, 80);
        self.left_stick.on_change.set(&self.level.player, |player, dir| {
            player.add_impulse(dir);
        });

        self.test_view = self.add_view();
        self.test_view
            .set_image(Image::get("cat.png"))
            .set_button_image(Image::get("square.png"))
            .set_animation_image(Image::get("palm.png"))
            .place()
            .br()
            .val(20)
            .size(280, 400);

        self.make_this(|this, view: &mut BaseView| {
            view.place()
                .bottom()
                .val(10)
                .center_hor()
                .size(150, 100)
                .all_ver();

            this.to_benchmark = view.add_view();
            this.to_benchmark.set_text("Benchmark");
            this.to_benchmark
                .on_tap
                .set(this, |this, _| this.ui.set_view::<BenchmarkView>());

            this.to_test = view.add_view();
            this.to_test.set_text("Test");
            this.to_test
                .on_tap
                .set(this, |this, _| this.ui.set_view::<UITestView>());

            this.play = view.add_view();
            this.play.set_text("Play sound");
            this.play.on_tap.set(this, |this, _| this.sound.play());

            this.async_task = view.add_view();
            this.async_task.set_text("Async task").set_frame((120, 20));
            this.async_task.on_tap.set(this, |this, _| {
                GET_USERS.get(this, |this, error, result| {
                    if let Some(error) = error {
                        infov!(&error);
                        this.alert(error);
                        return;
                    }

                    infov!(&result);

                    if let Some(user) = result.first() {
                        this.async_task.set_text(user.login.clone());
                    } else {
                        this.alert("No response");
                    }
                });
            });
        });

        self.sound = Sound::get("retro.wav");

        [self.to_benchmark, self.to_test, self.play, self.async_task].apply(|button| {
            button.set_color(Color::WHITE);
            button.set_corner_radius(8);
        });
    }
}

impl ViewCallbacks for TestGameView {
    fn setup(&mut self) {
        self.setup_ui();
        self.setup_level();
    }
}

impl MainView for TestGameView {
    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}

impl HasLevel for TestGameView {
    fn player(&self) -> Rglica<Player> {
        self.level.player
    }

    fn level(&self) -> Rglica<dyn Level> {
        (&self.level as &dyn Level).to_rglica()
    }
}
