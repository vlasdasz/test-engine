use std::ops::{Deref, DerefMut};

use gm::Rect;
use rapier2d::{geometry::ColliderHandle, prelude::ActiveEvents};
use rtools::{Rglica, ToRglica};

use crate::{Body, Level, Sprite, SpriteData};

#[derive(Debug)]
pub struct Unit {
    body: Body,
}

impl Unit {
    pub fn enable_collision_detection(&mut self) {
        self.collider_mut()
            .set_active_events(ActiveEvents::CONTACT_EVENTS);

        let rglica = (self as &dyn Sprite).to_rglica();
        self.level_mut().base_mut().colliding_sprites.push(rglica);
    }
}

impl Sprite for Unit {
    fn update(&mut self) {
        self.body.update()
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.body.collider_handle()
    }

    fn data(&self) -> &SpriteData {
        self.body.data()
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        self.body.data_mut()
    }

    fn make(rect: Rect, level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized,
    {
        let mut body = Body::make(rect, level);

        body.lock_rotations();
        body.collider_mut().set_restitution(0.0);

        Box::new(Unit {
            body: Box::into_inner(body),
        })
    }
}

impl Deref for Unit {
    type Target = Body;
    fn deref(&self) -> &Body {
        &self.body
    }
}

impl DerefMut for Unit {
    fn deref_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}
