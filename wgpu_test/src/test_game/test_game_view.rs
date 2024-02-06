use test_engine::{
    audio::Sound,
    manage::data_manager::DataManager,
    rtools::Apply,
    sprite_view::SpriteView,
    ui,
    ui::{refs::Weak, ViewData, ViewSubviews},
    ui_views::{test_view::ViewWithCat, Button, DPadView, IntView},
    view,
    wgpu_wrapper::image::Image,
};

use crate::{
    test_game::test_game_view::ui::{SubView, ViewSetup},
    Color, Container,
};

#[view]
pub struct TestGameView {
    dpad:        SubView<DPadView>,
    // left_stick:  SubView<AnalogStickView>,
    sprite_view: SubView<SpriteView>,
    test_view:   SubView<ViewWithCat>,

    ui_scale:    SubView<IntView>,
    level_scale: SubView<IntView>,

    sound: Weak<Sound>,
}

impl TestGameView {
    fn setup_ui(mut self: Weak<Self>) {
        self.sprite_view.place().tr(10).size(400, 80);

        self.dpad.place().size(140, 100).b(10).l(100);

        // self.left_stick.place().bl(10).size(80, 80);
        // self.left_stick.on_change.val(|dir| {
        //     dbg!(&dir);
        //     // if let Some(level) = &mut UILayer::get().level {
        //     //     dir.y = -dir.y;
        //     //     level.player().add_impulse(dir);
        //     // }
        // });

        self.test_view.place().br(20).size(280, 400);
        self.test_view
            .set_image(Image::get("cat.png"))
            .set_button_image(Image::get("square.png"))
            .set_animation_image(Image::get("palm.png"));

        self.ui_scale.step = 0.1;
        self.ui_scale.place().size(28, 120).l(100).b(140);
        //self.ui_scale.on_change.sub(|val| UIManager::set_ui_scale(val));

        self.level_scale.step = 0.1;
        self.level_scale.place().size(28, 120).l(28).b(140);
        // self.level_scale
        //     .on_change(|val| UILayer::get().level.as_mut().unwrap().set_scale(val));

        {
            let mut view = self.add_view::<Container>();

            view.place().b(10).center_x().size(150, 100).all_ver();

            let mut to_benchmark = view.add_view::<Button>();
            to_benchmark.set_text("Benchmark");
            // to_benchmark.on_tap(|| {
            //     UILayer::set_level(Own::<BenchmarkLevel>::default());
            // });

            let mut to_test = view.add_view::<Button>();
            to_test.set_text("Test");
            // to_test.on_tap(|| {
            //     UILayer::set_level(Own::<TestGameLevel>::default());
            // });

            let mut play = view.add_view::<Button>();
            play.set_text("Play sound");
            play.on_tap(move || self.sound.play());

            let mut screenshot = view.add_view::<Button>();
            screenshot.set_text("Screenshot");
            //screenshot.on_tap(Screen::take_screenshot);

            [to_benchmark, to_test, play, screenshot].apply(|mut button| {
                button.set_color(Color::WHITE);
                button.set_corner_radius(8);
            });
        }

        self.sound = Sound::get("retro.wav");
    }
}

impl ViewSetup for TestGameView {
    fn setup(self: Weak<Self>) {
        self.setup_ui();
        // self.setup_level();
    }
}
