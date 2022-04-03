use rtools::{Animation, Rglica, ToRglica, Unwrap};
use test_engine::{
    assets::Assets,
    sprites::{Player, Wall},
    Level, LevelBase, Sprite,
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
        let square = Assets::image("square.png");

        self.floor = self.add_wall((0, 0, 100, 4).into());
        self.floor.set_image(square.clone());

        self.left_wall = self.add_wall((-40, 0, 10, 100).into());
        self.left_wall.set_image(square.clone());

        self.right_wall = self.add_wall((40, 0, 10, 100).into());
        self.right_wall.set_image(square);

        self.left_animation = Animation::new(-60, -55, 10);
        self.right_animation = Animation::new(60, 55, 10);
        self.floor_animation = Animation::new(-10, 0, 4);
    }
}

impl Level for BenchmarkLevel {
    fn setup(&mut self) {
        let player: Box<Player> = Player::make(Assets::image("frisk.png"), self.rglica()).into();

        self.player = player.to_rglica();
        self.base.player = Unwrap::from_box(player);

        self.player.weapon.set_image(Assets::image("ak.png"));
        self.player.weapon.bullet_image = Assets::image("bullet.png").into();
        self.player.weapon.bullet_speed = 100.0;
        self.set_scale(1.2);
        self.make_walls();
    }

    fn update(&mut self) {
        self.player.weapon.shoot_at((10, 5).into());
        self.bullets_count += 1;
        self.left_wall.set_x(self.left_animation.value());
        self.right_wall.set_x(self.right_animation.value());
        // self.floor.set_y(self.floor_animation.value());
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
