use std::ops::{Deref, DerefMut};

use gm::flat::Point;
use rapier2d::{
    dynamics::{RigidBody, RigidBodyHandle},
    prelude::{Collider, ColliderHandle},
};
use refs::{MainLock, Own, Weak};
use smart_default::SmartDefault;
use wgpu_wrapper::WGPUApp;

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
    pub fn set_level<T: Level + 'static>(level: T) -> Weak<T> {
        let l = SELF.get_mut();
        let level = Own::new(level);
        let weak = level.weak();
        l.level = Some(level);
        l.level.as_ref().unwrap().__internal_setup();
        weak
    }

    pub fn stop_level() {
        SELF.get_mut().level = None;
        *Self::scale() = 1.0;
        *Self::camera_pos() = (0, 0).into();
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

    pub fn convert_touch(pos: Point) -> Point {
        let mut pos = pos;
        let size = WGPUApp::current().window_size;

        pos.x -= size.width / 2.0;
        pos.y -= size.height / 2.0;
        pos.y = -pos.y;
        pos /= 10;

        pos *= 2;
        pos /= WGPUApp::screen_scale();

        pos /= *Self::scale();

        pos += *Self::camera_pos();

        pos
    }
}
