use serde::{Deserialize, Serialize};
use tao_log::infov;
use test_engine::{
    audio::Sound,
    gm::{flat::Direction, Color},
    main_view::{HasLevel, MainView},
    net::{GetRequest, API},
    rtools::{
        data_manager::{DataManager, Handle},
        Apply, Rglica, ToRglica,
    },
    sprite_view::SpriteView,
    sprites::{Control, Player},
    ui::{
        basic::Button,
        complex::{AnalogStickView, IntView},
        test::test_view::TestView,
        view, BaseView, DPadView, SubView, View, ViewBase, ViewCallbacks, ViewData, ViewFrame, ViewLayout,
        ViewSubviews,
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
    dpad:        SubView<DPadView>,
    left_stick:  SubView<AnalogStickView>,
    sprite_view: SubView<SpriteView>,
    test_view:   SubView<TestView>,

    ui_scale:    SubView<IntView>,
    level_scale: SubView<IntView>,

    sound: Handle<Sound>,

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

        [" ", "w", "s", "d", "a"].apply2(
            [
                Direction::Up,
                Direction::Up,
                Direction::Down,
                Direction::Right,
                Direction::Left,
            ],
            |key, direction| {
                self.ui
                    .keymap
                    .add(key, self, move |this| this.player().move_by_direction(direction));
            },
        );

        self.set_frame((10, 10, 1000, 500));

        self.sprite_view.place().tr().val(10).size(400, 80);

        self.level
            .base()
            .on_sprite_selected
            .set(self, |this, sprite| this.sprite_view.set_sprite(sprite));

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

        self.left_stick.place().bl().val(10).size(80, 80);
        self.left_stick.on_change.set(&self.level.player, |player, dir| {
            player.add_impulse(dir);
        });

        self.test_view
            .set_image(Image::get("cat.png"))
            .set_button_image(Image::get("square.png"))
            .set_animation_image(Image::get("palm.png"))
            .place()
            .br()
            .val(20)
            .size(280, 400);

        self.ui_scale.step = 0.1;
        self.ui_scale
            .place()
            .size(28, 120)
            .left()
            .val(100)
            .bottom()
            .val(140);
        self.ui_scale
            .set_images(Image::get("up.png"), Image::get("down.png"));
        self.ui_scale
            .on_change
            .set(self, |this, val| this.ui.set_scale(val));

        self.level_scale.step = 0.1;
        self.level_scale
            .place()
            .size(28, 120)
            .left()
            .val(28)
            .bottom()
            .val(140);
        self.level_scale
            .set_images(Image::get("up.png"), Image::get("down.png"));
        self.level_scale
            .on_change
            .set(self, |this, val| this.level.set_scale(val));

        {
            let mut view = self.initialize_view::<BaseView>();

            view.place()
                .bottom()
                .val(10)
                .center_hor()
                .size(150, 100)
                .all_ver();

            let mut to_benchmark = view.initialize_view::<Button>();
            to_benchmark.set_text("Benchmark");
            to_benchmark
                .on_tap
                .set(self, |this, _| this.ui.set_view::<BenchmarkView>());

            let mut to_test = view.initialize_view::<Button>();
            to_test.set_text("Test");
            to_test
                .on_tap
                .set(self, |this, _| this.ui.set_view::<UITestView>());

            let mut play = view.initialize_view::<Button>();
            play.set_text("Play sound");
            play.on_tap.set(self, |this, _| this.sound.play());

            let mut async_task = view.initialize_view::<Button>();
            async_task.set_text("Async task").set_frame((120, 20));
            async_task.on_tap.set(self, move |this, _| {
                GET_USERS.get(this, |this, error, result| {
                    if let Some(error) = error {
                        infov!(&error);
                        this.alert(error);
                        return;
                    }

                    infov!(&result);

                    if let Some(_user) = result.first() {
                        // task.set_text(user.login.clone());
                    } else {
                        this.alert("No response");
                    }
                });
            });

            [to_benchmark, to_test, play, async_task].apply(|button| {
                button.set_color(Color::WHITE);
                button.set_corner_radius(8);
            });
        }

        self.sound = Sound::get("retro.wav");
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
