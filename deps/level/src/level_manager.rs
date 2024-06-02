use std::ops::{Deref, DerefMut};

use gm::flat::Point;
use rapier2d::{
    dynamics::{RigidBody, RigidBodyHandle},
    prelude::{Collider, ColliderHandle},
};
use refs::{MainLock, Own, Weak};
use smart_default::SmartDefault;

use crate::Level;

static SELF: MainLock<LevelManager> = MainLock::new();

#[derive(SmartDefault)]
pub struct LevelManager {
    #[default(1.0)]
    scale:      f32,
    camera_pos: Point,

    level: Option<Own<dyn Level>>,
}

impl LevelManager {
    pub fn update(frame_time: f32) {
        if Self::no_level() {
            return;
        }

        Self::level().__internal_update(frame_time);
    }
}

impl LevelManager {
    pub fn set_level(level: impl Level + 'static) {
        let l = SELF.get_mut();
        l.level = Some(Own::new(level));
        l.level.as_ref().unwrap().__internal_setup();
    }

    pub fn stop_level() {
        SELF.get_mut().level = None;
    }

    pub fn level() -> &'static dyn Level {
        SELF.level.as_ref().expect("No Level").deref()
    }

    pub fn level_weak() -> Weak<dyn Level> {
        SELF.level.as_ref().expect("No Level").weak()
    }

    pub fn downcast_level<T: Level + 'static>() -> Weak<T> {
        Self::level_weak().downcast::<T>().unwrap()
    }

    pub(crate) unsafe fn level_unchecked() -> &'static mut dyn Level {
        SELF.get_unchecked().level.as_mut().expect("No Level").deref_mut()
    }

    pub(crate) fn get_rigid_body(handle: RigidBodyHandle) -> &'static RigidBody {
        unsafe { &LevelManager::level_unchecked().sets.rigid_bodies[handle] }
    }

    pub(crate) fn get_collider(handle: ColliderHandle) -> &'static Collider {
        unsafe { &LevelManager::level_unchecked().sets.colliders[handle] }
    }

    pub fn no_level() -> bool {
        SELF.level.is_none()
    }

    pub fn scale() -> &'static mut f32 {
        &mut SELF.get_mut().scale
    }

    pub fn camera_pos() -> &'static mut Point {
        &mut SELF.get_mut().camera_pos
    }
}
