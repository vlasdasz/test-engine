use log::error;
use netrun::local_ip;
use test_engine::{
    AppRunner,
    audio::Sound,
    dispatch::after,
    gm::{Apply, Direction},
    level::{Control, LevelManager},
    refs::{Weak, manage::DataManager},
    ui::{
        Alert, Anchor::*, Button, ColorMeter, Container, DPadView, DrawingView, GREEN, ImageView, Label,
        MovableView, NoImage, NumberView, PURPLE, Point, PointsPath, Setup, Spinner, SpriteView, StickView,
        Style, TURQUOISE, TextField, UIManager, ViewData, ViewSubviews, view,
    },
};

use crate::{
    interface::test_game_view::MenuView,
    levels::{BenchmarkLevel, TestLevel},
    no_physics::NoPhysicsView,
};

pub(crate) static BUTTON: Style = Style::new(|btn| {
    btn.set_color((18, 208, 255));
    btn.set_corner_radius(5);
});

pub(crate) static HAS_BACK_BUTTON: Style = Style::new(|view| {
    view.add_view::<Button>()
        .add_transition::<Container, TestGameView>()
        .set_text("Back")
        .place()
        .size(100, 50)
        .t(200)
        .l(10);
});

#[view]
pub struct TestGameView {
    level: Weak<TestLevel>,

    #[init]
    ip:     Label,
    app_id: Label,
    system: Button,

    menu: MenuView,

    drawing: DrawingView,
    stick:   StickView,

    image: ImageView,

    label_l:  Label,
    image_r:  ImageView,
    gradient: Container,

    dpad:        DPadView,
    level_scale: NumberView,
    ui_scale:    NumberView,

    spinner: Button,
    alert:   Button,
    sound:   Button,

    color_meter: ColorMeter,

    text_field: TextField,

    test_level: Button,

    some_button: Button,

    sprite_view: MovableView<SpriteView>,
}

impl Setup for TestGameView {
    fn inspect(self: Weak<Self>) {
        dbg!("Test game view is: OK");
    }

    #[allow(clippy::too_many_lines)]
    fn setup(mut self: Weak<Self>) {
        //DebugView::enable();
        UIManager::root_view().set_image(NoImage);

        if false {
            UIManager::set_view(NoPhysicsView::new());
            return;
        }

        self.level = LevelManager::set_level(TestLevel::default());

        self.setup_keymap();

        self.ip
            .set_text(local_ip().map_or_else(
                |err| {
                    error!("{err}");
                    "Not supported".to_string()
                },
                |ip| ip.to_string(),
            ))
            .set_text_size(10);
        self.ip.place().t(20).l(10).size(80, 20);

        self.app_id.set_text(UIManager::app_instance_id());
        self.app_id.place().at_right(self.ip, 10);

        self.system.set_text("system");
        self.system.place().below(self.ip, 10);
        self.system.on_tap(|| {
            Alert::with_label(|l| {
                l.set_text_size(15);
            })
            .show(netrun::System::get_info().dump());
        });

        self.menu
            .place()
            .anchor(Top, self.system, 50)
            .same_x(self.system)
            .w(200)
            .anchor(Bot, self.dpad, 10);

        self.image.place().center_x().b(5).relative_size(self, 0.14);
        self.image.set_image("cat.png").set_corner_radius(20);

        self.label_l
            .place()
            .b(5)
            .relative_size(self.image, 1.0)
            .anchor(Right, self.image, 20);
        self.label_l.text = "Łėŵœ Ы".into();
        self.label_l.set_text_size(64.).set_corner_radius(20);

        self.image_r
            .place()
            .b(5)
            .relative_size(self.image, 1.0)
            .anchor(Left, self.image, 20);
        self.image_r.set_image("palm.png");

        self.gradient
            .place()
            .same([Height, Y], self.image_r)
            .w(50)
            .anchor(Left, self.image_r, 10);
        self.gradient.set_gradient(PURPLE, TURQUOISE);
        self.gradient.set_corner_radius(20);

        self.dpad.place().size(80, 65).b(10).l(10);

        self.dpad.on_press.val(move |direction| {
            self.level.player.unit.body.move_by_direction(direction);

            self.label_l.set_text(format!("{direction:?}"));
            AppRunner::set_window_title(format!("{direction:?}"));

            if direction.is_up() {
                AppRunner::set_window_title(format!("{direction:?} read pixel"));
            }
        });

        self.level_scale.place().at_right(self.dpad, 5).w(22);
        self.level_scale.set_min(4);
        self.level_scale.on_change(|val| {
            LevelManager::set_scale(val * 0.1);
        });

        self.ui_scale.place().at_right(self.level_scale, 5);
        self.ui_scale.set_min(4);
        self.ui_scale.set_value(10);
        self.ui_scale.on_change(|val| {
            UIManager::set_scale(val * 0.1);
        });

        self.spinner.place().size(150, 40).b(20).anchor(Left, self.ui_scale, 10);
        self.spinner.set_text("Spinner");
        self.spinner.set_text_size(20);
        self.spinner.on_tap(|| {
            let spin = Spinner::lock();
            after(2.0, || {
                spin.animated_stop();
            });
        });

        self.alert
            .place()
            .same_size(self.spinner)
            .anchor(Left, self.ui_scale, 10)
            .anchor(Bot, self.spinner, 10);
        self.alert.set_text("Alert");
        self.alert.set_text_size(20);
        self.alert.on_tap(|| {
            Alert::show("Hello!");
        });

        self.sound
            .place()
            .same_size(self.spinner)
            .anchor(Left, self.ui_scale, 10)
            .anchor(Bot, self.alert, 10);
        self.sound.set_text("Sound");
        self.sound.set_text_size(20);
        self.sound.on_tap(|| Sound::get("retro.wav").play());

        self.color_meter.place().size(50, 50).br(10);

        self.drawing.place().w(280).tr(10).relative(Height, self, 0.2);

        self.drawing.add_path([(0, 0), (40, 20), (20, 200), (150, 20), (20, 50)], GREEN);

        self.drawing
            .add_path(PointsPath::circle_triangles_with((200, 100), 50, 5), TURQUOISE);

        self.stick.place().t(40).size(200, 200).anchor(Right, self.drawing, 10);

        self.text_field.set_placeholder("type").place().below(self.system, 10).w(50);

        self.test_level.set_text("test level");
        self.test_level
            .place()
            .same([Y, Height], self.text_field)
            .w(100)
            .anchor(Left, self.text_field, 10);
        self.test_level.on_tap(|| {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(TestLevel::default());
        });

        self.sprite_view.set_title("Sprite:");
        self.sprite_view.place().size(280, 120).center_y().r(0);
        let player = self.level.player;
        self.sprite_view.set_sprite(player);
    }
}

impl TestGameView {
    fn setup_keymap(mut self: Weak<Self>) {
        [
            (' ', Direction::Up),
            ('w', Direction::Up),
            ('s', Direction::Down),
            ('d', Direction::Right),
            ('a', Direction::Left),
        ]
        .apply(|(key, direction)| {
            UIManager::keymap().add(self, key, move || {
                self.level.player.unit.body.move_by_direction(direction);
            });
        });

        UIManager::keymap().add(self, '=', || {
            LevelManager::set_scale(LevelManager::scale() * 2.0);
        });

        UIManager::keymap().add(self, '-', || {
            LevelManager::set_scale(LevelManager::scale() / 2.0);
        });

        UIManager::keymap().add(self, 'b', || {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(BenchmarkLevel::default());
        });
    }
}
