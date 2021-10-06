use test_engine::{Image, Level, LevelBase, Sprite};

#[derive(Default)]
pub struct TestLevel {
    base: LevelBase,
}

impl Level for TestLevel {
    fn setup(&mut self) {
        self.base.player = self.add_body((0, 10, 17.0 / 6.0, 28.0 / 6.0).into());
        self.base.player.lock_rotations();

        let square = Image::load(&test_engine::paths::images().join("square.png"));

        self.add_sprite((0, 0, 1, 1).into());
        self.add_wall((0, 0, 100, 1).into()).set_image(square);
        self.add_wall((20, 0, 1, 100).into()).set_image(square);
        self.add_wall((-20, 0, 1, 100).into()).set_image(square);

        for i in 0..500 {
            self.add_body((0.1 * i as f32, i * 2, 0.5, 0.5).into());
        }
    }

    fn level(&self) -> &LevelBase { &self.base }

    fn level_mut(&mut self) -> &mut LevelBase { &mut self.base }
}
