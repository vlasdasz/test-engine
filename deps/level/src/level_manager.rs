use std::ops::{Deref, DerefMut};

use gm::flat::Point;
use refs::{MainLock, Own};

use crate::Level;

static SELF: MainLock<LevelManager> = MainLock::new();

#[derive(Default)]
pub struct LevelManager {
    camera_pos: Point,

    level: Option<Own<dyn Level>>,
}

impl LevelManager {
    pub fn update() {
        if Self::no_level() {
            return;
        }
        Self::level_mut().base_mut().update_physics(1. / 60.);
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

    pub fn level() -> &'static dyn Level {
        SELF.level.as_ref().expect("No Level").deref()
    }

    pub fn level_mut() -> &'static mut dyn Level {
        SELF.get_mut().level.as_mut().expect("No Level").deref_mut()
    }

    pub fn no_level() -> bool {
        SELF.level.is_none()
    }

    pub fn camera_pos() -> Point {
        SELF.camera_pos
    }

    pub fn set_camera_pos(pos: impl Into<Point>) {
        SELF.get_mut().camera_pos = pos.into();
    }
}
