use std::ops::{Deref, DerefMut};

use refs::{MainLock, Own};

use crate::Level;

static SELF: MainLock<LevelManager> = MainLock::const_new();

#[derive(Default)]
pub struct LevelManager {
    level: Option<Own<dyn Level>>,
}

impl LevelManager {
    pub fn update() {
        if SELF.level.is_none() {
            return;
        }
        Self::level_mut().update();
    }
}

impl LevelManager {
    pub fn set_level(mut level: impl Level + 'static) {
        level.setup();
        SELF.get_mut().level = Some(Own::new(level));
    }

    pub fn level() -> &'static dyn Level {
        SELF.level.as_ref().expect("No Level").deref()
    }

    pub fn level_mut() -> &'static mut dyn Level {
        SELF.get_mut().level.as_mut().expect("No Level").deref_mut()
    }
}
