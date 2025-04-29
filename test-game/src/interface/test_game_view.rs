use test_engine::{
    AppRunner, DataManager, async_after,
    audio::Sound,
    gm::{Apply, Direction},
    level::{Control, LevelManager},
    refs::Weak,
    store::OnDisk,
    ui::{
        Alert, Anchor,
        Anchor::{Height, Left, Top, Width, X, Y},
        BLUE, Button, ColorMeter, Container, DPadView, DrawingView, GREEN, HasText, ImageView, Label,
        MovableView, NumberView, ORANGE, PURPLE, Point, PointsPath, PositionView, Setup, Spinner, SpriteView,
        StickView, Style, Switch, TURQUOISE, TextField, UIManager, ViewData, ViewFrame, ViewSubviews, WHITE,
        view,
    },
};
use ui_benchmark::BenchmarkView;

use crate::{
    interface::{
        game_view::GameView, noise_view::NoiseView, polygon_view::PolygonView, render_view::RenderView,
    },
    levels::{BenchmarkLevel, TestLevel},
    no_physics::NoPhysicsView,
};

static BOOL: OnDisk<bool> = OnDisk::new("bool");

pub(crate) static _BUTTON: Style = Style::new(|btn| {
    btn.set_color((18, 208, 255));
    btn.set_corner_radius(10);
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
    tl: Container,
    tr: Container,
    bl: Container,
    br: Container,

    drawing: DrawingView,
    stick:   StickView,

    image: ImageView,

    label_l:  Label,
    image_r:  ImageView,
    gradient: Container,

    dpad:  DPadView,
    scale: NumberView,

    spinner: Button,
    alert:   Button,
    sound:   Button,

    color_meter: ColorMeter,

    text_field: TextField,

    ui_bench: Button,

    render: Button,

    benchmark:  Button,
    test_level: Button,

    add_box: Button,

    position: PositionView,

    polygon: Button,
    noise:   Button,

    some_button: Button,

    sprite_view: MovableView<SpriteView>,

    bool_storage_view: Switch,

    no_physics: Button,

    game: Button,
}

impl Setup for TestGameView {
    #[allow(clippy::too_many_lines)]
    fn setup(mut self: Weak<Self>) {
        //DebugView::enable();

        if false {
            UIManager::set_view(NoPhysicsView::new());
            return;
        }

        self.level = LevelManager::set_level(TestLevel::default());

        self.setup_keymap();

        self.tl.set_color(PURPLE).place().size(100, 100).tl(10);
        self.tr.set_color(GREEN).place().size(100, 100).tr(10);
        self.bl.set_color(BLUE).place().size(100, 100).bl(10);
        self.br.set_color(ORANGE).place().size(100, 100).br(10);

        self.image.place().center_x().b(5).relative(Anchor::Size, self, 0.14);
        self.image.set_image("cat.png").set_corner_radius(20);

        self.label_l.place().b(5).relative(Anchor::Size, self.image, 1.0).anchor(
            Anchor::Right,
            self.image,
            20,
        );
        self.label_l.text = "Łėŵœ Ы".into();
        self.label_l.set_text_size(64.).set_corner_radius(20);

        self.image_r.place().b(5).relative(Anchor::Size, self.image, 1.0).anchor(
            Anchor::Left,
            self.image,
            20,
        );
        self.image_r.set_image("palm.png");

        self.gradient
            .place()
            .same([Height, Y], self.image_r)
            .w(50)
            .anchor(Left, self.image_r, 10);
        self.gradient.set_gradient(PURPLE, TURQUOISE);
        self.gradient.set_corner_radius(20);

        self.dpad.place().size(200, 140).b(20).anchor(Left, self.bl, 10);

        self.dpad.on_press.val(move |direction| {
            self.level.player.unit.body.move_by_direction(direction);

            self.label_l.set_text(format!("{direction:?}"));
            AppRunner::set_window_title(format!("{direction:?}"));

            if direction.is_up() {
                AppRunner::set_window_title(format!("{direction:?} read pixel"));
            }
        });

        self.scale.place().size(80, 150).b(20).anchor(Left, self.dpad, 10);
        self.scale.set_min(4.0);
        self.scale.on_change(|val| {
            *LevelManager::scale() = val * 0.1;
        });

        self.spinner.place().size(150, 40).b(20).anchor(Left, self.scale, 10);
        self.spinner.set_text("Spinner");
        self.spinner.set_text_size(20);
        self.spinner.on_tap(|| {
            let spin = Spinner::lock();
            async_after(4, async {
                spin.stop();
            });
        });

        self.alert.place().same_size(self.spinner).anchor(Left, self.scale, 10).anchor(
            Anchor::Bot,
            self.spinner,
            10,
        );
        self.alert.set_text("Alert");
        self.alert.set_text_size(20);
        self.alert.on_tap(|| {
            Alert::show("Hello!");
            AppRunner::set_window_size((600, 600))
        });

        self.sound.place().same_size(self.spinner).anchor(Left, self.scale, 10).anchor(
            Anchor::Bot,
            self.alert,
            10,
        );
        self.sound.set_text("Sound");
        self.sound.set_text_size(20);
        self.sound.on_tap(|| Sound::get("retro.wav").play());

        self.color_meter.place().size(100, 100).b(10).anchor(Anchor::Right, self.br, 10);

        self.drawing
            .place()
            .w(280)
            .t(10)
            .anchor(Anchor::Right, self.tr, 10)
            .relative(Height, self, 0.2);

        self.drawing.add_path([(0, 0), (40, 20), (20, 200), (150, 20), (20, 50)], GREEN);

        self.drawing
            .add_path(PointsPath::circle_triangles_with((200, 100), 50, 5), TURQUOISE);

        self.stick.place().t(40).size(200, 200).anchor(Anchor::Right, self.drawing, 10);

        self.text_field.set_placeholder("type");
        self.text_field.place().size(150, 50).t(200).anchor(Left, self.tl, 10);

        self.render.set_text("render");
        self.render.place().size(100, 50).t(200).anchor(Left, self.text_field, 10);
        self.render.on_tap(|| {
            LevelManager::stop_level();
            UIManager::set_view(RenderView::new());
        });

        self.benchmark.set_text("bench");
        self.benchmark.place().size(100, 50).t(200).anchor(Left, self.render, 10);
        self.benchmark.on_tap(|| {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(BenchmarkLevel::default());
        });

        self.test_level.set_text("test");
        self.test_level.place().size(100, 50).t(200).anchor(Left, self.benchmark, 10);
        self.test_level.on_tap(|| {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(TestLevel::default());
        });

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

        self.no_physics.set_text("No physics");
        self.no_physics
            .place()
            .same([Y, Height], self.bool_storage_view)
            .anchor(Left, self.bool_storage_view, 10)
            .w(250);
        self.no_physics.add_transition::<Self, NoPhysicsView>();

        self.game.set_text("Game");
        self.game
            .place()
            .same([Y, Height], self.no_physics)
            .anchor(Left, self.no_physics, 10)
            .w(100);
        self.game.on_tap(|| {
            LevelManager::stop_level();
            UIManager::set_view(GameView::new());
        });
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
}
