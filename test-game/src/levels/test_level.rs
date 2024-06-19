use fake::{Fake, Faker};
use test_engine::{
    audio::Sound,
    gen::noise::{generate_terrain, TerrainParams},
    gm::{LossyConvert, Shape},
    level::{
        level, Body, Level, LevelCreation, LevelManager, LevelSetup, Player, Sprite, SpriteTemplates, Wall,
    },
    refs::Weak,
    ui::{Color, Image, Point, Size},
    DataManager,
};

#[level]
#[derive(Default)]
pub struct TestLevel {
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
            bx.set_image("crate.png");
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

        self.background = Image::get("sky.png");

        self.make_sprite::<Wall>(Shape::Rect((200, 5).into()), (0, -5))
            .set_color(Color::random());
        // .set_image(render_text("oo spolokolkok", Font::helvetica().deref_mut(), 64));
        self.make_sprite::<Wall>(Shape::Rect((5, 100).into()), (120, 0))
            .set_image("square.png");
        self.make_sprite::<Wall>(Shape::Rect((5, 100).into()), (-120, 0))
            .set_image("square.png");

        self.make_sprite::<Body>(Shape::triangle((-5, -5), (5, -5), (-5, 5)), (0, 50))
            .set_image("triangle.png");

        self.make_sprite::<Body>(Shape::triangle((-5, -5), (5, -5), (-5, 5)), (0, 80))
            .set_color(Color::BLUE);

        for i in 0..150 {
            self.add_random_box((0.1f32 * i.lossy_convert(), i * 4 + 40));
        }

        let convex_points = vec![
            Point { x: -0.93, y: 6.39 },
            Point { x: -9.24, y: -1.83 },
            Point { x: 12.08, y: -1.41 },
            Point { x: 15.24, y: 3.65 },
        ];

        self.make_sprite::<Body>(Shape::Polygon(convex_points), (-20, 40))
            .set_color(Color::GREEN);

        let concave_points = vec![
            Point { x: -16.89, y: 4.16 },
            Point { x: 8.59, y: 11.09 },
            Point { x: 11.99, y: -0.36 },
            Point { x: -9.97, y: -9.34 },
            Point { x: -3.92, y: -0.85 },
        ];

        self.make_sprite::<Body>(Shape::Polygon(concave_points), (-20, 60))
            .set_color(Color::TURQUOISE);

        let mut player: Weak<Player> = self.make_sprite(Shape::Rect((1.2, 2).into()), (0, 20));
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

        self.make_sprite::<Wall>(Shape::Polygon(make_test_terrain()), (0, 20));
    }

    fn update(&mut self) {
        let pos = self.player.position();
        *LevelManager::camera_pos() = pos;
    }
}

pub fn make_test_terrain() -> Vec<Point> {
    let islands = generate_terrain(TerrainParams::default()).islands;
    let max_size = islands.iter().map(Vec::len).max().unwrap();
    islands.into_iter().find(|i| i.len() == max_size).unwrap()
}
