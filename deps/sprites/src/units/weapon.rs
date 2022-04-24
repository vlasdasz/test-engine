use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::flat::{Point, Shape};
use rtools::{data_manager::Handle, Rglica};

use crate::{level_base::LevelTemplates, sprite::SpriteTemplates, Body, Level, Sprite, SpriteData};

#[derive(Debug)]
pub struct Weapon {
    sprite:              SpriteData,
    pub(crate) velocity: Point,
    pub bullet_speed:    f32,
    pub bullet_image:    Handle<Image>,
    pub bullet_shape:    Shape,
}

impl Weapon {
    pub fn shoot_at(&mut self, pos: impl Into<Point>) {
        let vector = (pos.into() - self.position()).normalized();
        let pos = self.position() + vector * 4;

        let vel = vector * self.bullet_speed + self.velocity;

        let shape = self.bullet_shape;
        let mut bullet = self.level.add_sprite::<Body>(shape, pos);

        bullet.set_rotation(self.rotation());
        bullet.set_velocity(vel);
        bullet.set_restitution(0.5);
        bullet.data_mut().tag = "bullet".into();
        bullet.set_image(self.bullet_image);
    }
}

impl Sprite for Weapon {
    fn data(&self) -> &SpriteData {
        &self.sprite
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }

    fn make(shape: Shape, position: Point, level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self {
            sprite:       *SpriteData::make(shape, position, level),
            velocity:     Default::default(),
            bullet_speed: 1.0,
            bullet_image: Default::default(),
            bullet_shape: (1, 1).into(),
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
