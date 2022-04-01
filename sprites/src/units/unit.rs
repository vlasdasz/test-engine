use std::ops::{Deref, DerefMut};

use gl_image::Image;

use crate::{Body, Level, Sprite, SpriteBase};

#[derive(Debug)]
pub struct Unit {
    body: Body,
}

impl Unit {
    pub fn make(image: Image, level: &mut (impl Level + 'static)) -> Unit {
        let size = image.size.fit_height(2);
        let mut body = Body::make((0, 10, size.width, size.height).into(), level);
        body.lock_rotations();
        body.collider_mut().set_restitution(0.0);
        body.set_image(image);

        Unit { body }
    }
}

impl Sprite for Unit {
    fn update(&mut self) {
        self.body.update()
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
