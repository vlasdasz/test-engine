use fake::{Fake, Faker};
use test_engine::{
    DataManager,
    audio::Sound,
    generate::noise::{TerrainParams, generate_terrain},
    gm::{LossyConvert, Shape},
    level::{
        Banner, Body, Level, LevelCreation, LevelManager, LevelSetup, Player, Sprite, SpriteTemplates, Wall,
        level,
    },
    refs::Weak,
    ui::{BLUE, Color, GREEN, Image, Point, Size, TURQUOISE},
};

#[level]
#[derive(Default)]
pub struct TestLevel {
    pub player:      Weak<Player>,
    selected_sprite: Option<Weak<dyn Sprite>>,
    collision_sound: Weak<Sound>,
}

impl TestLevel {
    pub fn add_random_box(&mut self, pos: impl Into<Point>) {
        let mut bx = self.make_sprite::<Body>(
            Shape::Rect(Size::<f32>::new((0.2..2.8).fake(), (0.2..2.8).fake())),
            pos,
        );

        if Faker.fake() {
            // bx.set_image("crate_box.png");
            bx.set_image("svg_rendered.png");
        } else {
            bx.set_color(Color::random());
        }
    }

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

    fn add_player(&mut self) {
        let mut player: Weak<Player> = self.make_sprite(Shape::Rect((1.2, 2).into()), (-50, 60));
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

        self.make_sprite::<Wall>(Shape::Rect((10, 1).into()), (-50, 55));

        self.collision_sound = Sound::get("pek.wav");
    }

    fn add_house(&mut self) {
        self.make_sprite::<Wall>(Shape::Rect((20, 1).into()), (-65, 55));
        self.make_sprite::<Banner>(Shape::Rect((10, 10).into()), (-58, 60.5))
            .set_image("wood-window.png")
            .to_foreground();
        self.make_sprite::<Banner>(Shape::Rect((10, 10).into()), (-65, 60.5))
            .set_image("wood-window.png")
            .to_background();
    }
}

impl LevelSetup for TestLevel {
    fn needs_physics(&self) -> bool {
        true
    }

    fn setup(&mut self) {
        self.background = Image::get("sky.png");

        self.make_sprite::<Wall>(Shape::Rect((200, 5).into()), (0, -5))
            .set_color(Color::random());
        self.make_sprite::<Wall>(Shape::Rect((5, 100).into()), (100, 0))
            .set_image("square.png");
        self.make_sprite::<Wall>(Shape::Rect((5, 100).into()), (-100, 0))
            .set_image("square.png");

        self.make_sprite::<Body>(Shape::triangle((-5, -5), (5, -5), (-5, 5)), (0, 50))
            .set_image("triangle.png");

        self.make_sprite::<Body>(Shape::triangle((-5, -5), (5, -5), (-5, 5)), (-20, 80))
            .set_color(BLUE);

        let boxes = 100;

        for i in 0..boxes {
            let i = i * 2;
            let coeff: f32 = if i < boxes / 2 { -0.4 } else { 0.4 };
            self.add_random_box((coeff * i.lossy_convert(), i * 4 + 40));
        }

        let convex_points = vec![
            Point { x: -0.93, y: 6.39 },
            Point { x: -9.24, y: -1.83 },
            Point { x: 12.08, y: -1.41 },
            Point { x: 15.24, y: 3.65 },
        ];

        self.make_sprite::<Body>(Shape::Polygon(convex_points), (-20, 40))
            .set_color(GREEN);

        let concave_points = vec![
            Point { x: -16.89, y: 4.16 },
            Point { x: 8.59, y: 11.09 },
            Point { x: 11.99, y: -0.36 },
            Point { x: -9.97, y: -9.34 },
            Point { x: -3.92, y: -0.85 },
        ];

        self.make_sprite::<Body>(Shape::Polygon(concave_points), (-20, 60))
            .set_color(TURQUOISE);

        self.add_player();
        self.add_house();

        self.on_tap.val(move |pos| {
            LevelManager::level_weak()
                .as_any_mut()
                .downcast_mut::<Self>()
                .unwrap()
                .on_touch(pos);
        });

        for island in make_test_terrain() {
            self.make_sprite::<Wall>(Shape::Polyline(island), (0, 20));
        }
    }

    fn update(&mut self) {
        let pos = self.player.position();
        *LevelManager::camera_pos() = pos;
    }
}

pub fn make_test_terrain() -> Vec<Vec<Point>> {
    generate_terrain(TerrainParams::default()).islands
}
