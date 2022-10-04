use test_engine::{
    rtools::{data_manager::DataManager, Animation},
    sprites::{LevelCreation, Player, SpriteTemplates, Wall},
    Image, Level, LevelBase,
};
use ui::refs::{ToWeak, Weak};

#[derive(Default)]
pub struct BenchmarkLevel {
    base:       LevelBase,
    left_wall:  Weak<Wall>,
    right_wall: Weak<Wall>,
    floor:      Weak<Wall>,

    left_animation:  Animation,
    right_animation: Animation,
    floor_animation: Animation,

    pub player:        Weak<Player>,
    pub bullets_count: u64,
}

impl BenchmarkLevel {
    fn make_walls(&mut self) {
        let square = Image::get("square.png");

        self.floor = self.add_sprite((100, 10), (0, 0));
        self.floor.set_image(square);

        self.left_wall = self.add_sprite((10, 200), (-40, 0));
        self.left_wall.set_image(square);

        self.right_wall = self.add_sprite((10, 200), (40, 0));
        self.right_wall.set_image(square);

        self.left_animation = Animation::new(-60, -55, 10);
        self.right_animation = Animation::new(60, 55, 10);
        self.floor_animation = Animation::new(-10, 0, 4);
    }
}

impl Level for BenchmarkLevel {
    fn setup(&mut self) {
        self.player = self.add_sprite((2, 2), (0, 5));

        self.player.set_image(Image::get("frisk.png"));

        self.player.weapon.set_image(Image::get("ak.png"));
        self.player.weapon.bullet_image = Image::get("bullet.png");
        self.player.weapon.bullet_speed = 100.0;
        self.player.weapon.bullet_shape = (1, 0.28).into();

        self.set_scale(1.0);
        self.make_walls();
    }

    fn update(&mut self) {
        self.player.weapon.shoot_at((10, 15));
        self.bullets_count += 1;
        self.left_wall.set_x(self.left_animation.value());
        self.right_wall.set_x(self.right_animation.value());
    }

    fn base(&self) -> &LevelBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LevelBase {
        &mut self.base
    }

    fn rglica(&self) -> Weak<dyn Level> {
        (self as &dyn Level).weak()
    }
}
