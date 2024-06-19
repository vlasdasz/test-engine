use std::f32::consts::PI;

use test_engine::{
    gm::{LossyConvert, Shape},
    level::{level, Body, CoefficientCombineRule, LevelCreation, LevelSetup, Sensor, SpriteTemplates, Wall},
    ui::Color,
};

#[level]
#[derive(Default)]
pub struct FrictionLevel {}

impl LevelSetup for FrictionLevel {
    fn setup(&mut self) {
        self.make_sprite::<Wall>(Shape::rect(200, 2), (0, -80)).set_color(Color::BLACK);

        for i in 0..10 {
            let shift: f32 = i.lossy_convert() * 10.0f32;

            self.make_sprite::<Body>(Shape::rect(2, 2), (0, 0.0 + shift))
                .set_color(Color::GREEN)
                .set_friction(i.lossy_convert() / 28.0f32)
                .set_restitution(0.0, CoefficientCombineRule::Min)
                .tag = i;

            self.make_sprite::<Wall>(Shape::rect(40, 0.5), (0, -5.0 + shift))
                .set_rotation(PI / 10.0)
                .set_color(Color::BLACK);
        }

        self.make_sprite::<Sensor>(Shape::rect(28, 1), (-40, -20))
            .set_color(Color::ORANGE)
            .on_collision
            .val(|sprite| {
                dbg!(sprite.tag);
            });

        self.make_sprite::<Sensor>(Shape::rect(1, 4), (-18, 44))
            .set_color(Color::ORANGE)
            .on_collision
            .val(|sprite| {
                dbg!(sprite.tag);
            });
    }

    fn update(&mut self) {
        self.update_physics(0.05);
        self.update_physics(0.05);
        self.update_physics(0.05);
        self.update_physics(0.05);
        self.update_physics(0.05);
    }
}
