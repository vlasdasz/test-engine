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

        let floor = Wall::make((0, 0, 100, 10).into(), self.rglica());
        self.floor = floor.to_rglica();
        self.floor.set_image(square.clone());
        self.add_sprite(floor);

        let left_wall = Wall::make((-40, 0, 10, 100).into(), self.rglica());
        self.left_wall = left_wall.to_rglica();
        self.left_wall.set_image(square.clone());
        self.add_sprite(left_wall);

        let right_wall = Wall::make((40, 0, 10, 100).into(), self.rglica());
        self.right_wall = right_wall.to_rglica();
        self.right_wall.set_image(square);
        self.add_sprite(right_wall);

        self.left_animation = Animation::new(-60, -55, 10);
        self.right_animation = Animation::new(60, 55, 10);
        self.floor_animation = Animation::new(-10, 0, 4);
    }
}

impl Level for BenchmarkLevel {
    fn setup(&mut self) {
        let player = Player::make((0, 5, 2, 2).into(), self.rglica());

        self.player = player.to_rglica();
        self.player.set_image(Assets::image("frisk.png"));

        self.base.player = Unwrap::from_box(player);

        self.player.weapon.set_image(Assets::image("ak.png"));
        self.player.weapon.bullet_image = Assets::image("bullet.png").into();
        self.player.weapon.bullet_speed = 100.0;
        self.set_scale(1.0);
        self.make_walls();
    }

    fn update(&mut self) {
        self.player.weapon.shoot_at((10, 5).into());
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

    fn rglica(&self) -> Rglica<dyn Level> {
        (self as &dyn Level).to_rglica()
    }
}
