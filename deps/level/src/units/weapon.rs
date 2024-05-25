use std::ops::{Deref, DerefMut};

use gm::{
    flat::{Point, Shape},
    Color,
};
use rapier2d::dynamics::CoefficientCombineRule;
use refs::{Own, Weak};
use wgpu_wrapper::image::Image;

use crate::{Body, LevelCreation, LevelManager, Sprite, SpriteData, SpriteTemplates};

pub struct Weapon {
    sprite:              SpriteData,
    pub(crate) velocity: Point,
    pub bullet_speed:    f32,
    pub bullet_image:    Weak<Image>,
    pub bullet_shape:    Shape,
}

impl Weapon {
    pub fn shoot_at(self: Weak<Self>, pos: impl Into<Point>) {
        let vector = (pos.into() - self.position()).normalized();
        let pos = self.position() + vector * 4;

        let vel = vector * self.bullet_speed + self.velocity;

        let shape = self.bullet_shape.clone();
        let mut bullet = LevelManager::level_mut().add_sprite::<Body>(shape, pos);

        bullet.set_rotation(self.rotation());
        bullet.set_velocity(vel);
        bullet.set_restitution(0.5, CoefficientCombineRule::Average);
        bullet.set_color(Color::random());
        bullet.set_image(self.bullet_image);
    }
}

impl Sprite for Weapon {
    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized {
        Own::new(Self {
            sprite:       SpriteData::make(shape, position),
            velocity:     Default::default(),
            bullet_speed: 1.0,
            bullet_image: Default::default(),
            bullet_shape: Shape::Rect((1, 1).into()),
        })
    }
}

impl Deref for Weapon {
    type Target = SpriteData;
    fn deref(&self) -> &SpriteData {
        &self.sprite
    }
}

impl DerefMut for Weapon {
    fn deref_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }
}
