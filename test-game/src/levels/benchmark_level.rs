use test_engine::{
    gm::{Animation, Shape},
    level::{level, LevelCreation, LevelSetup, Player, Sprite, SpriteTemplates, Wall},
    refs::Weak,
    ui::{Alert, Color, Image, UIManager},
    DataManager,
};

use crate::levels::make_test_terrain;

#[level]
#[derive(Default)]
pub struct BenchmarkLevel {
    low_frames: usize,

    top_wall:   Weak<Wall>,
    left_wall:  Weak<Wall>,
    right_wall: Weak<Wall>,
    floor:      Weak<Wall>,

    bottom_moving: Weak<Wall>,

    left_animation:   Animation,
    right_animation:  Animation,
    floor_animation:  Animation,
    bottom_animation: Animation,

    finish: bool,

    pub player:        Weak<Player>,
    pub bullets_count: u64,
}

impl BenchmarkLevel {
    fn make_walls(&mut self) {
        let square = Image::get("square.png");

        self.top_wall = self.make_sprite(Shape::Rect((100, 5).into()), (0, 110));
        self.top_wall.set_color(Color::random());

        self.floor = self.make_sprite(Shape::Rect((100, 5).into()), (0, 0));
        self.floor.set_image(square);

        self.left_wall = self.make_sprite(Shape::Rect((5, 50).into()), (-40, 0));
        self.left_wall.set_image(square);

        self.right_wall = self.make_sprite(Shape::Rect((5, 50).into()), (40, 0));
        self.right_wall.set_image(square);

        self.bottom_moving = self.make_sprite(Shape::rect(5, 14), (0, -68));
        self.bottom_moving.set_image(square);

        self.left_animation = Animation::new(-80.0, -20.0, 2.0);
        self.right_animation = Animation::new(80.0, 20.0, 2.0);
        self.floor_animation = Animation::new(-25.0, 0.0, 0.5);
        self.bottom_animation = Animation::new(-100.0, 100.0, 4.0);

        self.make_sprite::<Wall>(Shape::rect(200, 2), (0, -85)).set_image(square);
        self.make_sprite::<Wall>(Shape::rect(2, 200), (120, 0)).set_image(square);
        self.make_sprite::<Wall>(Shape::rect(2, 200), (-120, 0)).set_image(square);

        for island in make_test_terrain() {
            self.make_sprite::<Wall>(Shape::Polygon(island), (-20, 0));
        }
    }
}

impl LevelSetup for BenchmarkLevel {
    fn setup(&mut self) {
        self.background = Image::get("sky.png");

        self.player = self.make_sprite(Shape::Rect((2, 2).into()), (0, 5));
        self.player.set_color(Color::random());

        self.player.set_image("frisk.png");

        self.player.weapon.set_image("ak.png");
        self.player.weapon.bullet_image = Image::get("bullet.png");
        self.player.weapon.bullet_speed = 100.0;
        self.player.weapon.bullet_shape = Shape::Rect((1, 0.28).into());

        self.make_walls();
    }

    fn update(&mut self) {
        self.left_wall.set_x(self.left_animation.value());
        self.right_wall.set_x(self.right_animation.value());
        self.floor.set_y(self.floor_animation.value());
        self.bottom_moving.set_x(self.bottom_animation.value());

        if self.finish {
            return;
        }

        if UIManager::fps() < 40.0 {
            self.low_frames += 1;
        }

        if self.low_frames >= 120 {
            self.finish = true;
            Alert::show(format!("{} sprites", self.bullets_count));
        }

        self.player.weapon.weak().shoot_at((0, 15));
        self.player.weapon.weak().shoot_at((10, 15));
        self.player.weapon.weak().shoot_at((15, 10));
        self.player.weapon.weak().shoot_at((-10, 15));
        self.player.weapon.weak().shoot_at((-15, 10));
        self.bullets_count += 5;
    }
}
