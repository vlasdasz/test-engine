use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::{dynamics::CoefficientCombineRule, prelude::ActiveEvents};
use refs::{weak_from_ref, Own};

use crate::{Body, LevelManager, Sprite, SpriteData};

pub struct Unit {
    pub body: Own<Body>,
}

impl Unit {
    pub fn enable_collision_detection(&mut self) -> &mut Self {
        self.collider_mut().set_active_events(ActiveEvents::COLLISION_EVENTS);
        let weak = weak_from_ref(self);
        LevelManager::level_mut().colliding_sprites.push(weak);
        self
    }
}

impl Sprite for Unit {
    fn update(&mut self) {
        self.body.update()
    }

    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized {
        let mut body = Body::make(shape, position);

        body.lock_rotations();
        body.collider_mut().set_restitution(0.0);
        body.collider_mut().set_restitution_combine_rule(CoefficientCombineRule::Min);

        Own::new(Unit { body })
    }
}

impl Deref for Unit {
    type Target = SpriteData;
    fn deref(&self) -> &SpriteData {
        &self.body
    }
}

impl DerefMut for Unit {
    fn deref_mut(&mut self) -> &mut SpriteData {
        &mut self.body
    }
}
