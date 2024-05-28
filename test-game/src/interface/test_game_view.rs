use std::num::NonZeroU32;

use test_engine::{
    async_after,
    audio::Sound,
    gm::{Apply, Direction, LossyConvert},
    level::{Control, LevelManager},
    refs::Weak,
    ui::{
        link_button, view, Alert, Anchor, Button, Color, ColorMeter, Container, DPadView, DrawingView,
        ImageView, Label, NumberView, Point, PointsPath, Spinner, StickView, Sub, TextField, UIManager,
        ViewData, ViewSetup,
    },
    App, DataManager,
};

use crate::levels::{BenchmarkLevel, TestLevel};

#[view]
pub struct TestGameView {
    tl: Sub<Container>,
    tr: Sub<Container>,
    bl: Sub<Container>,
    br: Sub<Container>,

    drawing: Sub<DrawingView>,
    stick:   Sub<StickView>,

    image: Sub<ImageView>,

    label_l: Sub<Label>,
    image_r: Sub<ImageView>,

    dpad:  Sub<DPadView>,
    scale: Sub<NumberView<NonZeroU32>>,

    spinner: Sub<Button>,
    alert:   Sub<Button>,
    sound:   Sub<Button>,

    color_meter: Sub<ColorMeter>,

    text_field: Sub<TextField>,

    objc: Sub<Button>,

    benchmark:  Sub<Button>,
    test_level: Sub<Button>,
}

impl ViewSetup for TestGameView {
    fn setup(mut self: Weak<Self>) {
        LevelManager::set_level(TestLevel::default());

        self.setup_keymap();

        self.tl.set_color(Color::PURPLE).place().size(100, 100).tl(10);
        self.tr.set_color(Color::GREEN).place().size(100, 100).tr(10);
        self.bl.set_color(Color::BLUE).place().size(100, 100).bl(10);
        self.br.set_color(Color::ORANGE).place().size(100, 100).br(10);

        self.image.place().center_x().b(5).relative(Anchor::Size, self, 0.14);
        self.image.set_image("cat.png");

        self.label_l.place().b(5).relative(Anchor::Size, self.image, 1.0).anchor(
            Anchor::Right,
            self.image,
            20,
        );
        self.label_l.text = "Łėŵœ Ы".into();
        self.label_l.set_text_size(64.);

        self.image_r.place().b(5).relative(Anchor::Size, self.image, 1.0).anchor(
            Anchor::Left,
            self.image,
            20,
        );
        self.image_r.set_image("palm.png");

        self.dpad.place().size(200, 140).b(20).anchor(Anchor::Left, self.bl, 10);

        self.dpad.on_press.val(move |direction| {
            LevelManager::level_mut().player.unit.body.move_by_direction(direction);

            self.label_l.set_text(format!("{direction:?}"));
            App::set_window_title(format!("{direction:?}"));

            if direction.is_up() {
                App::set_window_title(format!("{direction:?} read pixel"));
            }
        });

        self.scale.place().size(80, 150).b(20).anchor(Anchor::Left, self.dpad, 10);
        self.scale.set_min(4.try_into().unwrap());
        self.scale.on_change(|val| {
            *LevelManager::scale() = val.get().lossy_convert() * 0.1;
        });

        self.spinner.place().size(100, 28).b(20).anchor(Anchor::Left, self.scale, 10);
        self.spinner.set_text("Spinner");
        self.spinner.set_text_size(20);
        self.spinner.on_tap(|| {
            Spinner::start();
            async_after(4, async {
                Spinner::stop();
            });
        });

        self.alert.place().size(100, 28).anchor(Anchor::Left, self.scale, 10).anchor(
            Anchor::Bot,
            self.spinner,
            10,
        );
        self.alert.set_text("Alert");
        self.alert.set_text_size(20);
        self.alert.on_tap(|| {
            Alert::show("Hello!");
            App::set_window_size((600, 600))
        });

        self.sound
            .place()
            .same_size(self.alert)
            .anchor(Anchor::Left, self.scale, 10)
            .anchor(Anchor::Bot, self.alert, 10);
        self.sound.set_text("Sound");
        self.sound.set_text_size(20);
        self.sound.on_tap(|| Sound::get("retro.wav").play());

        self.color_meter.place().size(100, 100).b(10).anchor(Anchor::Right, self.br, 10);

        self.drawing.place().w(280).t(10).anchor(Anchor::Right, self.tr, 10).relative(
            Anchor::Height,
            self,
            0.2,
        );

        self.drawing
            .add_path([(0, 0), (40, 20), (20, 200), (150, 20), (20, 50)], Color::GREEN);

        self.drawing.add_path(
            PointsPath::circle_triangles_with((200, 100), 50, 5),
            Color::TURQUOISE,
        );

        self.stick.place().t(40).size(200, 200).anchor(Anchor::Right, self.drawing, 10);

        self.text_field.set_placeholder("Type here");
        self.text_field.place().size(200, 50).t(200).anchor(Anchor::Left, self.tl, 10);

        self.objc.set_text("objc");
        link_button!(self, objc, call_obj);
        self.objc.place().size(100, 50).t(200).anchor(Anchor::Left, self.text_field, 10);

        self.benchmark.set_text("bench");
        self.benchmark.place().size(100, 50).t(200).anchor(Anchor::Left, self.objc, 10);
        self.benchmark.on_tap(|| {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(BenchmarkLevel::default());
        });

        self.test_level.set_text("test");
        self.test_level
            .place()
            .size(100, 50)
            .t(200)
            .anchor(Anchor::Left, self.benchmark, 10);
        self.test_level.on_tap(|| {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(TestLevel::default());
        });
    }
}

impl TestGameView {
    fn setup_keymap(self: Weak<Self>) {
        [
            (' ', Direction::Up),
            ('w', Direction::Up),
            ('s', Direction::Down),
            ('d', Direction::Right),
            ('a', Direction::Left),
        ]
        .apply(|(key, direction)| {
            UIManager::keymap().add(self, key, move || {
                LevelManager::level_mut().player.unit.body.move_by_direction(direction);
            });
        });

        UIManager::keymap().add(self, '=', || {
            *LevelManager::scale() *= 2.0;
        });

        UIManager::keymap().add(self, '-', || {
            *LevelManager::scale() /= 2.0;
        });

        UIManager::keymap().add(self, 'b', || {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(BenchmarkLevel::default());
        });
    }

    fn call_obj(self: Weak<Self>) {
        dbg!(&self.view_label);
    }
}
