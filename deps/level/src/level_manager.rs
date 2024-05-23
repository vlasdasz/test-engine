use std::ops::{Deref, DerefMut};

use gm::flat::Point;
use refs::{MainLock, Own};
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
        Self::level_mut().update_camera();
        Self::level_mut().base_mut().update_physics(frame_time);
        Self::level_mut().update();
    }
}

impl LevelManager {
    pub fn set_level(level: impl Level + 'static) {
        let level = Own::new(level);
        let mut weak = level.weak();
        SELF.get_mut().level = Some(level);
        weak.setup();
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
