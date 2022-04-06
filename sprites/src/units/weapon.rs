use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::flat::{Point, Shape};
use rtools::Rglica;

use crate::{add_sprite, Body, Level, Sprite, SpriteData};

#[derive(Debug)]
pub struct Weapon {
    sprite:              SpriteData,
    pub(crate) velocity: Point,
    pub bullet_speed:    f32,
    pub bullet_image:    Option<Image>,
}

impl Weapon {
    pub fn shoot_at(&mut self, pos: impl Into<Point>) {
        let vector = (pos.into() - self.position()).normalized();
        let pos = self.position() + vector * 4;

        let vel = vector * self.bullet_speed + self.velocity;

        let mut bullet = add_sprite::<Body>(0.5, pos, self.level.deref_mut());

        bullet.set_rotation(self.rotation());
        bullet.set_velocity(vel);
        bullet.data_mut().tag = "bullet".into();

        if let Some(image) = &self.bullet_image {
            bullet.set_image(image.clone())
        }
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
            sprite:       SpriteData::make(shape, position).with_level(level),
            velocity:     Default::default(),
            bullet_speed: 1.0,
            bullet_image: None,
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
