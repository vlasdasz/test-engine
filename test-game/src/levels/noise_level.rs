use test_engine::{
    gm::Shape,
    level::{level, Body, LevelCreation, LevelSetup, Wall},
    refs::Weak,
    ui::Point,
};

#[level]
#[derive(Default)]
pub struct NoiseLevel {}

impl NoiseLevel {
    pub fn add_islands(mut self: Weak<Self>, islands: Vec<Vec<Point>>) {
        self.remove_all_sprites();
        self.add_sprite::<Wall>(Shape::rect(200, 2), (0, -80));

        for island in islands {
            self.add_sprite::<Body>(
                Shape::Polygon(island.into_iter().map(|p| (p.x, p.y + 40.0).into()).collect()),
                (0, 0),
            );
        }
    }
}

impl LevelSetup for NoiseLevel {
    fn setup(&mut self) {
        self.add_sprite::<Wall>(Shape::rect(200, 2), (0, -80));
    }
}
