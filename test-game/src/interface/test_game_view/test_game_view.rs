use log::error;
use netrun::local_ip;
use test_engine::{
    AppRunner,
    gm::{Apply, Direction},
    level::{Control, LevelManager},
    refs::Weak,
    ui::{
        Anchor::{Bot, Top},
        Button, ColorMeter, Container, DPadView, Label, NoImage, Point, Setup, StickView, Style, UIManager,
        ViewData, ViewSubviews, view,
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

    menu: MenuView,

    stick: StickView,

    dpad: DPadView,

    color_meter: ColorMeter,
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
        self.ip.place().t(10).l(5).size(80, 20);

        self.app_id.set_text(UIManager::app_instance_id());
        self.app_id.place().at_right(self.ip, 10);

        self.menu
            .place()
            .anchor(Top, self.ip, 10)
            .same_x(self.ip)
            .w(170)
            .anchor(Bot, self.dpad, 10);

        self.dpad.place().size(80, 65).b(10).l(10);

        self.dpad.on_press.val(move |direction| {
            self.level.player.unit.body.move_by_direction(direction);

            AppRunner::set_window_title(format!("{direction:?}"));

            if direction.is_up() {
                AppRunner::set_window_title(format!("{direction:?} read pixel"));
            }
        });

        self.color_meter.place().size(50, 50).tr(10);

        self.stick.place().br(20).size(150, 150);
        self.stick.on_change.val(move |direction| {
            self.level.player.unit.body.add_impulse(direction.invert_y() / 500.0);
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
