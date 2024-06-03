use std::f32::consts::PI;

use test_engine::{
    gm::Shape,
    level::{level, Body, CoefficientCombineRule, LevelCreation, LevelSetup, SpriteTemplates, Wall},
    ui::Color,
};

#[level]
#[derive(Default)]
pub struct FrictionLevel {}

impl LevelSetup for FrictionLevel {
    fn setup(&mut self) {
        self.add_sprite::<Wall>(Shape::rect(200, 2), (0, -80)).set_color(Color::BLACK);

        for i in 0..10 {
            let shift: f32 = i as f32 * 10.0;

            // self.add_sprite::<Body>(Shape::Circle(2.0), (10, 5.0 + shift))
            //     .set_image("ball.png");

            self.add_sprite::<Body>(Shape::rect(2, 2), (0, 0.0 + shift))
                .set_color(Color::GREEN)
                .set_friction(i as f32 / 20.0)
                .set_restitution(0.0, CoefficientCombineRule::Min);

            self.add_sprite::<Wall>(Shape::rect(40, 0.5), (0, -5.0 + shift))
                .set_rotation(PI / 10.0)
                .set_color(Color::BLACK);
        }
    }
}
