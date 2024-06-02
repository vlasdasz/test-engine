use test_engine::{
    audio::Sound,
    gm::{LossyConvert, Shape},
    level::{
        level, Body, Level, LevelCreation, LevelManager, LevelSetup, Player, Sprite, SpriteTemplates, Wall,
    },
    refs::Weak,
    ui::{Color, Point},
    DataManager,
};

#[level]
#[derive(Default)]
pub struct TestLevel {
    selected_sprite: Option<Weak<dyn Sprite>>,
    collision_sound: Weak<Sound>,
}

impl TestLevel {
    fn on_touch(&mut self, pos: Point) {
        if let Some(mut sprite) = self.sprite_at(pos) {
            sprite.set_selected(true);
            self.on_sprite_selected.trigger(sprite);
            if let Some(mut old) = self.selected_sprite {
                old.set_selected(false);
            }
            self.selected_sprite = sprite.into();
            return;
        }

        if let Some(mut sprite) = self.selected_sprite {
            sprite.set_selected(false);
            self.selected_sprite = None;
            self.on_sprite_selected.trigger(Weak::default());
        }
    }
}

impl LevelSetup for TestLevel {
    fn setup(&mut self) {
        // let drawn = Image::render("test_draw", (100, 100), |image| {
        //     GLWrapper::set_clear_color(Color::GREEN);
        //     GLWrapper::clear();
        //     GLWrapper::scissor((5, 5, 20, 20), || {
        //         GLWrapper::set_clear_color(Color::TURQUOISE);
        //         GLWrapper::clear();
        //     });
        //     GLWrapper::set_clear_color(Color::GRAY);
        //     image.channels = 1;
        // });

        // self.add_rect((30, 30, 40, 25)).set_image(drawn);

        self.add_sprite::<Wall>(Shape::Rect((100, 5).into()), (0, -5))
            .set_color(Color::random());
        // .set_image(render_text("oo spolokolkok", Font::helvetica().deref_mut(), 64));
        self.add_sprite::<Wall>(Shape::Rect((5, 100).into()), (60, 0))
            .set_image("square.png");
        self.add_sprite::<Wall>(Shape::Rect((5, 100).into()), (-60, 0))
            .set_image("square.png");

        // self.add_sprite::<Body>(Shape::triangle((-10, -10), (10, -10), (-10, 10)),
        // (0, 50))     .set_image("triangle.png");

        let _concave_points: Vec<Point> = vec![
            (5, -5).into(),
            (-10, -10).into(),
            (10, -10).into(),
            (10, 10).into(),
        ];

        // self.add_sprite::<Body>(Shape::Polygon(concave_points), (0, 100))
        //     .set_image("triangle.png");

        for i in 0..150 {
            self.add_sprite::<Body>(
                Shape::Rect((0.6, 0.6).into()),
                (0.1f32 * i.lossy_convert(), i * 2),
            )
            .set_image("square.png");
        }

        let mut player: Weak<Player> = self.add_sprite(Shape::Rect((1.2, 2).into()), (0, 5));
        self.player = player;
        player.set_image("frisk.png").unit.enable_collision_detection();
        player.weapon.set_image("ak.png");

        player.on_collision.sub(move || {
            LevelManager::level_weak()
                .as_any_mut()
                .downcast_mut::<Self>()
                .unwrap()
                .collision_sound
                .play();
        });

        self.collision_sound = Sound::get("pek.wav");

        self.on_tap.val(move |pos| {
            LevelManager::level_weak()
                .as_any_mut()
                .downcast_mut::<Self>()
                .unwrap()
                .on_touch(pos);
        });
    }

    fn update(&mut self) {
        let pos = self.player.position();
        *LevelManager::camera_pos() = pos;
    }
}
