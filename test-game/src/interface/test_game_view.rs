use std::sync::LazyLock;

use log::error;
use netrun::local_ip;
use test_engine::{
    AppRunner, Event,
    audio::Sound,
    dispatch::{after, on_main},
    filesystem::Paths,
    gm::{Apply, Direction},
    level::{Control, LevelManager},
    refs::{Weak, manage::DataManager},
    store::OnDisk,
    ui::{
        ALL_VIEWS, Alert,
        Anchor::{self, Height, Left, Top, Width, X, Y},
        Button, ColorMeter, Container, DPadView, DrawingView, GREEN, HasText, ImageView, Label, MovableView,
        NoImage, NumberView, PURPLE, Point, PointsPath, PositionView, Setup, Spinner, SpriteView, StickView,
        Style, Switch, TURQUOISE, TextField, UIManager, ViewData, ViewFrame, ViewSubviews, WHITE,
        all_view_tests, all_views, async_link_button, view,
    },
};
use ui_benchmark::BenchmarkView;

use crate::{
    api::TEST_REST_REQUEST,
    interface::{
        game_view::GameView, noise_view::NoiseView, polygon_view::PolygonView, render_view::RenderView,
        root_layout_view::RootLayoutView,
    },
    levels::{BenchmarkLevel, TestLevel},
    no_physics::NoPhysicsView,
};

static BOOL: LazyLock<OnDisk<bool>> = LazyLock::new(|| OnDisk::new("bool"));

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
    level:       Weak<TestLevel>,
    rest_tapped: Event<usize>,

    #[init]
    ip:       Label,
    app_id:   Label,
    sys_info: Button,

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

    ui_bench: Button,

    render: Button,

    benchmark:   Button,
    test_level:  Button,
    pick_folder: Button,
    all_views:   Button,

    add_box: Button,

    position: PositionView,

    polygon: Button,
    noise:   Button,
    panic:   Button,
    rest:    Button,

    some_button: Button,

    sprite_view: MovableView<SpriteView>,

    bool_storage_view: Switch,

    no_physics: Button,

    game:      Button,
    root_view: Button,
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
        self.ip.place().tl(20).size(80, 20);

        self.app_id.set_text(UIManager::app_instance_id());
        self.app_id.place().at_right(self.ip, 10);

        self.sys_info.set_text("system");
        self.sys_info.place().below(self.ip, 10);
        self.sys_info.on_tap(|| {
            Alert::with_label(|l| {
                l.set_text_size(15);
            })
            .show(netrun::System::get_info().dump());
        });

        self.image.place().center_x().b(5).relative_size(self, 0.14);
        self.image.set_image("cat.png").set_corner_radius(20);

        self.label_l
            .place()
            .b(5)
            .relative_size(self.image, 1.0)
            .anchor(Anchor::Right, self.image, 20);
        self.label_l.text = "Łėŵœ Ы".into();
        self.label_l.set_text_size(64.).set_corner_radius(20);

        self.image_r
            .place()
            .b(5)
            .relative_size(self.image, 1.0)
            .anchor(Anchor::Left, self.image, 20);
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
        self.ui_scale.on_change(|val| {
            UIManager::set_scale(val * 0.1);
        });
        self.ui_scale.set_value(10);

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
            .anchor(Anchor::Bot, self.spinner, 10);
        self.alert.set_text("Alert");
        self.alert.set_text_size(20);
        self.alert.on_tap(|| {
            Alert::show("Hello!");
        });

        self.sound
            .place()
            .same_size(self.spinner)
            .anchor(Left, self.ui_scale, 10)
            .anchor(Anchor::Bot, self.alert, 10);
        self.sound.set_text("Sound");
        self.sound.set_text_size(20);
        self.sound.on_tap(|| Sound::get("retro.wav").play());

        self.color_meter.place().size(50, 50).br(10);

        self.drawing.place().w(280).tr(10).relative(Height, self, 0.2);

        self.drawing.add_path([(0, 0), (40, 20), (20, 200), (150, 20), (20, 50)], GREEN);

        self.drawing
            .add_path(PointsPath::circle_triangles_with((200, 100), 50, 5), TURQUOISE);

        self.stick.place().t(40).size(200, 200).anchor(Anchor::Right, self.drawing, 10);

        self.text_field.set_placeholder("type").place().below(self.sys_info, 10).w(50);

        self.render
            .set_text("render")
            .on_tap(|| {
                LevelManager::stop_level();
                UIManager::set_view(RenderView::new());
            })
            .place()
            .at_right(self.text_field, 5);

        self.benchmark.set_text("bench");
        self.benchmark
            .place()
            .same([Y, Height], self.text_field)
            .w(100)
            .anchor(Left, self.render, 10);
        self.benchmark.on_tap(|| {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(BenchmarkLevel::default());
        });

        self.test_level.set_text("test level");
        self.test_level
            .place()
            .same([Y, Height], self.text_field)
            .w(100)
            .anchor(Left, self.benchmark, 10);
        self.test_level.on_tap(|| {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(TestLevel::default());
        });

        self.pick_folder.set_text("pick folder");
        self.pick_folder.place().at_right(self.test_level, 10);
        self.pick_folder.on_tap(|| {
            test_engine::dispatch::spawn(async {
                Alert::show(format!("{:?}", Paths::pick_folder().await));
            });
        });

        self.all_views
            .on_tap(|| {
                dbg!(all_views!());
                dbg!(ALL_VIEWS);
                dbg!(all_view_tests!());

                // dbg!(__)
            })
            .set_text("all views")
            .place()
            .at_right(self.pick_folder, 10);

        self.ui_bench.set_text("ui bench");
        self.ui_bench
            .place()
            .anchor(Top, self.text_field, 10)
            .same([X, Width, Height], self.text_field);
        self.ui_bench.on_tap(|| {
            LevelManager::stop_level();
            UIManager::set_view(BenchmarkView::new());
        });

        self.add_box.set_text("add box");
        self.add_box
            .place()
            .anchor(Left, self.ui_bench, 10)
            .same([Y, Width, Height], self.ui_bench);
        self.add_box.on_tap(move || {
            let mut level = LevelManager::downcast_level::<TestLevel>();
            let pos = LevelManager::convert_touch(self.position.frame().origin);
            level.add_random_box(pos);
        });

        self.position.set_position((400, 400));

        self.polygon.set_text("polygon");
        self.polygon
            .place()
            .anchor(Left, self.add_box, 10)
            .same([Y, Width, Height], self.add_box);
        self.polygon.on_tap(|| {
            UIManager::set_view(PolygonView::new());
        });

        self.noise.set_text("noise");
        self.noise
            .place()
            .anchor(Left, self.polygon, 10)
            .same([Y, Width, Height], self.polygon);
        self.noise.on_tap(|| {
            LevelManager::stop_level();
            UIManager::set_view(NoiseView::new().on_back(|| {
                UIManager::set_view(Self::new());
            }));
        });

        self.panic.set_text("panic");
        self.panic
            .place()
            .anchor(Left, self.noise, 10)
            .same([Y, Width, Height], self.noise);
        self.panic.on_tap(|| {
            panic!("test panic");
        });

        self.rest.set_text("request");
        self.rest
            .place()
            .anchor(Left, self.panic, 10)
            .same([Y, Width, Height], self.panic);
        async_link_button!(self.rest, rest_pressed);

        self.rest_tapped.val_async(move |val| async move {
            on_main(move || {
                self.rest.set_text(format!("rest: {val}"));
            });
        });

        self.sprite_view.set_title("Sprite:");
        self.sprite_view.place().size(280, 120).center_y().r(0);
        let player = self.level.player;
        self.sprite_view.set_sprite(player);

        self.bool_storage_view
            .set_off_color(WHITE)
            .set_on(BOOL.get().unwrap_or_default());
        self.bool_storage_view
            .place()
            .same([X, Height], self.ui_bench)
            .anchor(Top, self.ui_bench, 10)
            .w(100);
        self.bool_storage_view.selected.val(|val| {
            BOOL.set(val);
        });

        self.no_physics.set_text("no physics");
        self.no_physics
            .place()
            .same([Y, Height], self.bool_storage_view)
            .anchor(Left, self.bool_storage_view, 10)
            .w(250);
        self.no_physics.add_transition::<Self, NoPhysicsView>();

        self.game.set_text("game");
        self.game
            .place()
            .same([Y, Height], self.no_physics)
            .anchor(Left, self.no_physics, 10)
            .w(100);
        self.game.on_tap(|| {
            LevelManager::stop_level();
            UIManager::set_view(GameView::new());
        });

        self.root_view.set_text("root view");
        self.root_view
            .place()
            .same([Y, Height], self.no_physics)
            .anchor(Left, self.game, 10)
            .w(150);
        self.root_view.on_tap(|| {
            LevelManager::stop_level();
            UIManager::set_view(RootLayoutView::new());
        });
    }
}

impl TestGameView {
    async fn rest_pressed(self: Weak<Self>) -> anyhow::Result<()> {
        let spin = Spinner::lock();

        let users = TEST_REST_REQUEST.await?;

        spin.stop();

        Alert::show(format!(
            "Got {} users. First name: {}",
            users.len(),
            users.first().unwrap().name
        ));

        self.rest_tapped.trigger(users.len());

        Ok(())
    }

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
