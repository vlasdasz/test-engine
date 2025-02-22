#![allow(incomplete_features)]
#![feature(let_chains)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod app_runner;
mod assets;
mod assets_paths;
mod level_drawer;

mod app;
mod app_starter;
pub mod ui;

pub use app::App;
pub use app_starter::*;
pub use educe;
pub use ui::ui_test;

pub mod level {
    pub use ::level::{
        Banner, Body, CoefficientCombineRule, Control, Level, LevelBase, LevelCreation, LevelInternal,
        LevelManager, LevelSetup, LevelTemplates, Player, Sensor, Sprite, SpriteData, SpriteTemplates, Wall,
        level,
    };
}

pub mod refs {
    pub use refs::{
        AsAny, MainLock, Own, Weak, current_thread_id, dump_ref_stats, enable_ref_stats_counter,
        set_current_thread_as_main, vec::OwnVec, weak_from_ref,
    };
}

pub mod reflected {
    pub use ::reflected::{Field, Reflected, ToReflectedString, ToReflectedVal, Type};
}

pub mod gm {
    pub use gm::{
        Animation, Apply, LossyConvert, Platform, ToF32,
        flat::{Direction, Shape},
        sign::Sign,
        volume::GyroData,
    };
}

pub mod store {
    pub(crate) use store;
    pub use store::{EncryptionKey, OnDisk, OnDiskEncrypted, Paths};
}

pub mod time {
    pub use web_time::*;
}

pub use app_runner::AppRunner;
pub use audio;
pub use dispatch::{after, async_after, from_main, on_main, wait_for_next_frame};
pub use generate;
pub use manage::data_manager::DataManager;
pub use vents::{DelayedEvent, Event, OnceEvent};
pub use window::{RenderPass, VertexBuffer, Window, cast_slice, image::ToImage};

#[cfg(target_os = "android")]
pub type AndroidApp = winit::platform::android::activity::AndroidApp;
#[cfg(target_os = "android")]
pub type EventLoop = winit::event_loop::EventLoop<window::Events>;
