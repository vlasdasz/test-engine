use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::prelude::ActiveEvents;
use refs::{Own, Weak};

use crate::{Body, Level, Sprite, SpriteData};

pub struct Unit {
    body: Own<Body>,
}

impl Unit {
    pub fn enable_collision_detection(self: Weak<Self>) -> Weak<Self> {
        self.collider_mut().set_active_events(ActiveEvents::COLLISION_EVENTS);

        self.level_mut().base_mut().colliding_sprites.push(self);
        self
    }
}

impl Sprite for Unit {
    fn update(&mut self) {
        self.body.update()
    }

    fn data(&self) -> &SpriteData {
        self.body.data()
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        self.body.data_mut()
    }

    fn make(shape: Shape, position: Point, level: Weak<dyn Level>) -> Own<Self>
    where Self: Sized {
        let mut body = Body::make(shape, position, level);

        body.lock_rotations();
        body.collider_mut().set_restitution(0.0);

        Own::new(Unit { body })
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
