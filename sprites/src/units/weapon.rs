use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::{Point, Rect};
use rtools::Rglica;

use crate::{Body, Level, Sprite, SpriteData};

#[derive(Debug)]
pub struct Weapon {
    sprite:              SpriteData,
    pub(crate) velocity: Point,
    pub bullet_speed:    f32,
    pub bullet_image:    Option<Image>,
}

impl Weapon {
    pub fn shoot_at(&mut self, pos: Point) {
        let vector = (pos - self.position()).normalized();
        let pos = self.position() + vector * 3.2;

        let vel = vector * self.bullet_speed + self.velocity;

        //let mut body = self.level_mut().add_body((pos.x, pos.y, 0.8, 0.15).into());

        let mut body = Body::make((pos.x, pos.y, 0.8, 0.15).into(), self.level);

        body.set_rotation(self.rotation());
        body.set_velocity(vel);
        body.data_mut().tag = "bullet".into();

        if let Some(image) = &self.bullet_image {
            body.set_image(image.clone())
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

    fn make(rect: Rect, level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized,
    {
        let mut sprite = SpriteData::from(rect);
        sprite.level = level;
        Box::new(Self {
            sprite,
            velocity: Default::default(),
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
