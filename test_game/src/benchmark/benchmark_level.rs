use test_engine::{assets::Assets, sprites::Player, Level, LevelBase, Sprite};

#[derive(Debug, Default)]
pub struct BenchmarkLevel {
    level:             LevelBase,
    pub bullets_count: u64,
}

impl BenchmarkLevel {
    fn make_walls(&mut self) {
        let square = Assets::image("square.png");

        self.add_wall((0, 0, 100, 1).into()).set_image(square.clone());
        self.add_wall((40, 0, 1, 100).into()).set_image(square.clone());
        self.add_wall((-40, 0, 1, 100).into()).set_image(square);
    }
}

impl Level for BenchmarkLevel {
    fn setup(&mut self) {
        self.level.player = Player::make(Assets::image("frisk.png"), self.level_mut()).into();
        self.level.player.weapon.set_image(Assets::image("ak.png"));
        self.level.player.weapon.bullet_image = Assets::image("bullet.png").into();
        self.level.player.weapon.bullet_speed = 100.0;
        self.set_scale(0.8);
        self.make_walls();
    }

    fn update(&mut self) {
        self.player().weapon.shoot_at((10, 5).into());
        self.bullets_count += 1;
    }

    fn level(&self) -> &LevelBase {
        &self.level
    }

    fn level_mut(&mut self) -> &mut LevelBase {
        &mut self.level
    }
}
