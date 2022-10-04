use rtools::{static_default, Apply};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use test_engine::{
    audio::Sound,
    gm::{flat::Direction, Color},
    net::{GetRequest, API},
    rtools::data_manager::{DataManager, Handle},
    sprite_view::SpriteView,
    sprites::Control,
    view, Image, LevelBase, Screen,
};
use ui::{
    refs::{Own, Strong},
    BaseView, SubView, UIManager, ViewCallbacks, ViewData, ViewFrame, ViewSubviews,
};
use ui_views::{test_view::TestView, AnalogStickView, Button, DPadView, IntView};

use crate::{benchmark::BenchmarkLevel, BenchmarkView};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login:    String,
    password: String,
}

#[derive(SmartDefault)]
struct Network {
    #[default(API::get_request("get_users"))]
    _get_users: GetRequest<Vec<User>>,
}
static_default!(Network);

#[view]
#[derive(Default)]
pub struct TestGameView {
    dpad:        SubView<DPadView>,
    left_stick:  SubView<AnalogStickView>,
    sprite_view: SubView<SpriteView>,
    test_view:   SubView<TestView>,

    ui_scale:    SubView<IntView>,
    level_scale: SubView<IntView>,

    sound: Handle<Sound>,
}

impl TestGameView {
    fn setup_level(&mut self) {
        self.dpad
            .on_press
            .sub(|dir| Screen::current().ui.level.as_mut().unwrap().player().move_by_direction(dir));
    }

    fn setup_ui(&mut self) {
        Screen::current().ui.keymap.add('=', self, |_| {
            let scale = Screen::current().ui.scale() * 1.2;
            Screen::current().ui.set_scale(scale);
        });

        Screen::current().ui.keymap.add('-', self, |_| {
            let scale = Screen::current().ui.scale() * 0.8;
            Screen::current().ui.set_scale(scale);
        });

        [' ', 'w', 's', 'd', 'a'].apply2(
            [
                Direction::Up,
                Direction::Up,
                Direction::Down,
                Direction::Right,
                Direction::Left,
            ],
            |key, direction| {
                Screen::current().ui.keymap.add(*key, self, move |_| {
                    if let Some(level) = &mut Screen::current().ui.level {
                        if let Some(player) = level.player().get() {
                            player.move_by_direction(direction)
                        }
                    }
                });
            },
        );

        self.sprite_view.place.tr(10).size(400, 80);

        // if let Some(level) = &Screen::current().ui.level {
        //     level
        //         .base()
        //         .on_sprite_selected
        //         .set(self, |this, sprite| this.sprite_view.set_sprite(sprite));
        // }

        self.dpad.place.size(140, 100).b(10).l(100);
        self.dpad.set_images(
            Image::get("up.png"),
            Image::get("down.png"),
            Image::get("left.png"),
            Image::get("right.png"),
        );

        self.left_stick.place.bl(10).size(80, 80);
        self.left_stick.on_change.sub(|mut dir| {
            if let Some(level) = &mut Screen::current().ui.level {
                dir.y = -dir.y;
                level.player().add_impulse(dir);
            }
        });

        self.test_view.place.br(20).size(280, 400);
        self.test_view
            .set_image(Image::get("cat.png"))
            .set_button_image(Image::get("square.png"))
            .set_animation_image(Image::get("palm.png"));

        self.ui_scale.step = 0.1;
        self.ui_scale.place.size(28, 120).l(100).b(140);
        self.ui_scale.set_images(Image::get("up.png"), Image::get("down.png"));
        self.ui_scale.on_change.sub(|val| Screen::current().ui.set_scale(val));

        self.level_scale.step = 0.1;
        self.level_scale.place.size(28, 120).l(28).b(140);
        self.level_scale.set_images(Image::get("up.png"), Image::get("down.png"));
        self.level_scale
            .on_change
            .sub(|val| Screen::current().ui.level.as_mut().unwrap().set_scale(val));

        {
            let mut view = self.initialize_view::<BaseView>();

            view.place.b(10).center_hor().size(150, 100).all_ver();

            let mut to_benchmark = view.initialize_view::<Button>();
            to_benchmark.set_text("Benchmark");
            to_benchmark.on_tap.sub(|_| {
                Screen::current().ui.set_level(Strong::<BenchmarkLevel>::default());
                UIManager::set_view(Own::<BenchmarkView>::default());
            });

            let mut to_test = view.initialize_view::<Button>();
            to_test.set_text("Test");
            to_test.on_tap.sub(|_| {
                Screen::current().ui.set_level(Strong::<LevelBase>::default());
                UIManager::set_view(Own::<BenchmarkView>::default());
            });

            let mut play = view.initialize_view::<Button>();
            play.set_text("Play sound");
            // play.on_tap.set(self, |this, _| this.sound.play());

            let mut async_task = view.initialize_view::<Button>();
            async_task.set_text("Async task").set_frame((120, 20));
            // async_task.on_tap.set(self, move |this, _| {
            //     Network::get().get_users.get(this, |_, error, result| {
            //         if let Some(error) = error {
            //             infov!(&error);
            //             Alert::show(error);
            //             return;
            //         }
            //
            //         infov!(&result);
            //
            //         if let Some(_user) = result.first() {
            //             // task.set_text(user.login.clone());
            //         } else {
            //             Alert::show("No response");
            //         }
            //     });
            // });

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
