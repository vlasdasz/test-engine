use std::ops::{Deref, DerefMut};

use educe::Educe;
use gm::{flat::Point, LossyConvert, Platform};
use rapier2d::{
    dynamics::{RigidBody, RigidBodyHandle},
    prelude::{Collider, ColliderHandle},
};
use refs::{MainLock, Own, Weak};
use wgpu_wrapper::WGPUApp;

use crate::Level;

static SELF: MainLock<LevelManager> = MainLock::new();

#[derive(Educe)]
#[educe(Default)]
pub struct LevelManager {
    #[educe(Default = 1.0)]
    scale:      f32,
    camera_pos: Point,

    #[educe(Default = 1.0 / 60.0)]
    update_interval: f32,

    level: Option<Own<dyn Level>>,
}

impl LevelManager {
    pub const fn default_z_position() -> f32 {
        0.85
    }

    pub const fn z_position_offset() -> f32 {
        0.000_001
    }

    pub fn update() {
        if Self::no_level() {
            return;
        }

        Self::level().__internal_update(*Self::update_interval());
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

    pub fn update_interval() -> &'static mut f32 {
        &mut SELF.get_mut().update_interval
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
        pos /= 10.0;

        pos *= 2;

        if Platform::WIN {
            pos /= WGPUApp::screen_scale().ceil().lossy_convert();
        } else {
            pos /= WGPUApp::screen_scale().lossy_convert();
        }

        pos /= *Self::scale();

        pos += *Self::camera_pos();

        pos
    }
}
