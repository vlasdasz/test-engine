use manage::{data_manager::DataManager, handle::Handle};
use rtools::Apply;
use test_engine::{
    audio::Sound,
    gm::{flat::Direction, Color},
    sprite_view::SpriteView,
    sprites::Control,
    ui_layer::UILayer,
    view, Screen,
};
use ui::{
    refs::{Own, Weak},
    Container, SubView, ViewData, ViewSetup, ViewSubviews,
};
use ui_views::{test_view::ViewWithCat, AnalogStickView, Button, DPadView, IntView};

use crate::{benchmark::BenchmarkLevel, test_game::TestGameLevel};

#[view]
pub struct TestGameView {
    dpad:        SubView<DPadView>,
    left_stick:  SubView<AnalogStickView>,
    sprite_view: SubView<SpriteView>,
    test_view:   SubView<ViewWithCat>,

    ui_scale:    SubView<IntView>,
    level_scale: SubView<IntView>,

    sound: Handle<Sound>,
}

impl TestGameView {
    fn setup_level(&mut self) {
        // Screen::current().

        UILayer::set_level(Own::<TestGameLevel>::default());

        self.dpad
            .on_press
            .val(|dir| UILayer::get().level.as_mut().unwrap().player().move_by_direction(&dir));
    }

    fn setup_ui(mut self: Weak<Self>) {
        // Screen::current().ui.keymap.add('=', self, |_| {
        //     let scale = UIManager::ui_scale() * 1.2;
        //     UIManager::set_ui_scale(scale);
        // });
        //
        // Screen::current().ui.keymap.add('-', self, |_| {
        //     let scale = UIManager::ui_scale() * 0.8;
        //     UIManager::set_ui_scale(scale);
        // });

        [
            (' ', Direction::Up),
            ('w', Direction::Up),
            ('s', Direction::Down),
            ('d', Direction::Right),
            ('a', Direction::Left),
        ]
        .apply(|(key, direction)| {
            UILayer::keymap().add(key, move || {
                if let Some(level) = &mut UILayer::get().level {
                    if let Some(player) = level.player().get() {
                        player.move_by_direction(&direction)
                    }
                }
            });
        });

        self.sprite_view.place.tr(10).size(400, 80);

        if let Some(level) = &UILayer::get().level {
            level
                .base()
                .on_sprite_selected
                .val(move |sprite| self.sprite_view.set_sprite(sprite));
        }

        self.dpad.place.size(140, 100).b(10).l(100);

        self.left_stick.place.bl(10).size(80, 80);
        self.left_stick.on_change.val(|mut dir| {
            if let Some(level) = &mut UILayer::get().level {
                dir.y = -dir.y;
                level.player().add_impulse(dir);
            }
        });

        self.test_view.place.br(20).size(280, 400);
        self.test_view
            .set_image("cat.png")
            .set_button_image("square.png")
            .set_animation_image("palm.png");

        self.ui_scale.step = 0.1;
        self.ui_scale.place.size(28, 120).l(100).b(140);
        //self.ui_scale.on_change.sub(|val| UIManager::set_ui_scale(val));

        self.level_scale.step = 0.1;
        self.level_scale.place.size(28, 120).l(28).b(140);
        self.level_scale
            .on_change
            .val(|val| UILayer::get().level.as_mut().unwrap().set_scale(val));

        {
            let mut view = self.add_view::<Container>();

            view.place.b(10).center_x().size(150, 100).all_ver();

            let mut to_benchmark = view.add_view::<Button>();
            to_benchmark.set_text("Benchmark");
            to_benchmark.on_tap(|| {
                UILayer::set_level(Own::<BenchmarkLevel>::default());
            });

            let mut to_test = view.add_view::<Button>();
            to_test.set_text("Test");
            to_test.on_tap(|| {
                UILayer::set_level(Own::<TestGameLevel>::default());
            });

            let mut play = view.add_view::<Button>();
            play.set_text("Play sound");
            play.on_tap(move || self.sound.play());

            let mut screenshot = view.add_view::<Button>();
            screenshot.set_text("Screenshot");
            screenshot.on_tap(Screen::take_screenshot);

            [to_benchmark, to_test, play, screenshot].apply(|mut button| {
                button.set_color(Color::WHITE);
                button.set_corner_radius(8);
            });
        }

        self.sound = Sound::get("retro.wav");
    }
}

impl ViewSetup for TestGameView {
    fn setup(mut self: Weak<Self>) {
        self.setup_ui();
        self.setup_level();
    }
}
