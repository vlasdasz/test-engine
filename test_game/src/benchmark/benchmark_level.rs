use rtools::{data_manager::DataManager, Animation, Rglica, ToRglica};
use test_engine::{
    sprites::{add_sprite, Player, Wall},
    Image, Level, LevelBase, Sprite,
};

#[derive(Debug, Default)]
pub struct BenchmarkLevel {
    base:              LevelBase,
    left_wall:         Rglica<Wall>,
    right_wall:        Rglica<Wall>,
    floor:             Rglica<Wall>,
    left_animation:    Animation,
    right_animation:   Animation,
    floor_animation:   Animation,
    pub player:        Rglica<Player>,
    pub bullets_count: u64,
}

impl BenchmarkLevel {
    fn make_walls(&mut self) {
        let square = Image::get("square.png");

        self.floor = add_sprite((100, 10), (0, 0), self);
        self.floor.set_image(square.clone());

        self.left_wall = add_sprite((10, 100), (-40, 0), self);
        self.left_wall.set_image(square.clone());

        self.right_wall = add_sprite((10, 100), (40, 0), self);
        self.right_wall.set_image(square);

        self.left_animation = Animation::new(-60, -55, 10);
        self.right_animation = Animation::new(60, 55, 10);
        self.floor_animation = Animation::new(-10, 0, 4);
    }
}

impl Level for BenchmarkLevel {
    fn setup(&mut self) {
        self.player = add_sprite((2, 2), (0, 5), self);

        self.player.set_image(Image::get("frisk.png"));

        self.player.weapon.set_image(Image::get("ak.png"));
        self.player.weapon.bullet_image = Image::get("bullet.png").into();
        self.player.weapon.bullet_speed = 100.0;

        self.set_scale(1.0);
        self.make_walls();
    }

    fn update(&mut self) {
        self.player.weapon.shoot_at((10, 15));
        self.bullets_count += 1;
        self.left_wall.set_x(self.left_animation.value());
        self.right_wall.set_x(self.right_animation.value());
    }

    fn on_key_pressed(&mut self, key: String) {
        if key == "-" {
            self.set_scale(self.scale() / 2.0);
        } else if key == "=" {
            self.set_scale(self.scale() * 2.0);
        }
    }

    fn base(&self) -> &LevelBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LevelBase {
        &mut self.base
    }

    fn rglica(&self) -> Rglica<dyn Level> {
        (self as &dyn Level).to_rglica()
    }
}
