use std::ops::{Deref, DerefMut};

use gl_image::Image;
use rapier2d::{geometry::ColliderHandle, prelude::ActiveEvents};
use rtools::{Rglica, ToRglica};

use crate::{Body, Level, Sprite, SpriteBase};

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

    pub fn make(image: Image, level: Rglica<dyn Level>) -> Unit {
        let size = image.size.fit_height(2);
        let mut body = Body::make((0, 10, size.width, size.height).into(), level);

        body.lock_rotations();
        body.collider_mut().set_restitution(0.0);
        body.set_image(image);

        Unit {
            body: Box::into_inner(body),
        }
    }
}

impl Sprite for Unit {
    fn update(&mut self) {
        self.body.update()
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.body.collider_handle()
    }

    fn sprite(&self) -> &SpriteBase {
        self.body.sprite()
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        self.body.sprite_mut()
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
