#![allow(incomplete_features)]
#![feature(let_chains)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod app;
mod assets;
mod paths;

mod te_level;
pub mod ui;

pub use educe;
pub use ui::ui_test;

pub mod level {
    pub use ::level::{
        level, Body, CoefficientCombineRule, Control, Level, LevelBase, LevelCreation, LevelInternal,
        LevelManager, LevelSetup, LevelTemplates, Object, Player, Sensor, Sprite, SpriteData,
        SpriteTemplates, Wall,
    };
}

pub mod refs {
    pub use refs::{
        current_thread_id, dump_ref_stats, enable_ref_stats_counter, set_current_thread_as_main, vec::OwnVec,
        weak_from_ref, AsAny, MainLock, Own, Weak,
    };
}

pub mod reflected {
    pub use ::reflected::{Field, Reflected, ToReflectedString, ToReflectedVal, Type};
}

pub mod gm {
    pub use gm::{
        flat::{Direction, Shape},
        sign::Sign,
        volume::GyroData,
        Animation, Apply, LossyConvert, Platform, ToF32,
    };
}

pub mod store {
    pub(crate) use store;
    pub use store::{EncryptionKey, OnDisk, OnDiskEncrypted};
}

pub mod time {
    pub use web_time::*;
}

pub use app::App;
pub use audio;
pub use dispatch::{after, async_after, from_main, on_main, wait_for_next_frame};
pub use gen;
pub use manage::data_manager::DataManager;
pub use paths::*;
pub use store::store::{executable_name, OnDisk};
pub use vents::{DelayedEvent, Event, OnceEvent};
pub use wgpu_wrapper::{
    cast_slice, image::ToImage, RenderPass, SpriteRenderView, VertexBuffer, WGPUApp, WGPUDrawer,
};

#[cfg(target_os = "android")]
pub type AndroidApp = winit::platform::android::activity::AndroidApp;
#[cfg(target_os = "android")]
pub type EventLoop = winit::event_loop::EventLoop<wgpu_wrapper::Events>;
