use std::ops::{Deref, DerefMut};

use gm::flat::Point;
use rapier2d::{
    dynamics::{RigidBody, RigidBodyHandle},
    prelude::{Collider, ColliderHandle},
};
use refs::MainLock;
use smart_default::SmartDefault;

use crate::Level;

static SELF: MainLock<LevelManager> = MainLock::new();

#[derive(SmartDefault)]
pub struct LevelManager {
    #[default(1.0)]
    scale:      f32,
    camera_pos: Point,

    level: Option<Box<dyn Level>>,
}

impl LevelManager {
    pub fn update(frame_time: f32) {
        if Self::no_level() {
            return;
        }
        Self::level_mut().update_camera();
        Self::level_mut().update_physics(frame_time);
        Self::level_mut().update();
    }
}

impl LevelManager {
    pub fn set_level(level: impl Level + 'static) {
        let l = SELF.get_mut();
        l.level = Some(Box::new(level));
        l.level.as_mut().unwrap().setup();
    }

    pub fn stop_level() {
        SELF.get_mut().level = None;
    }

    pub fn level() -> &'static dyn Level {
        SELF.level.as_ref().expect("No Level").deref()
    }

    pub fn level_mut() -> &'static mut dyn Level {
        SELF.get_mut().level.as_mut().expect("No Level").deref_mut()
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
