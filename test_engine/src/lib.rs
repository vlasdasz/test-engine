#![allow(incomplete_features)]
#![feature(let_chains)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod app;
mod assets;
mod paths;
mod views;

pub mod ui {
    pub use ::ui::*;
    pub use gm::{
        flat::{Point, Points, PointsPath, Size},
        Color, U8Color,
    };
    pub use ui_proc::view;
    pub use ui_views::*;
    pub use views::color_meter::ColorMeter;
    pub use wgpu_wrapper::{image::Image, PolygonMode, Screenshot};

    use crate::views;
}

pub mod refs {
    pub use refs::{
        current_thread_id, set_current_thread_as_main,
        vec::{OwnVec, ToOwnVec},
        weak_from_ref, AsAny, Own, ToOwn, Weak,
    };
}

pub mod reflect {
    pub use reflected::{FieldRef, Reflected};
}

pub mod input {
    pub use winit::event::KeyEvent;
}

pub mod gm {
    pub use gm::{sign::Sign, Apply, IntoF32, LossyConvert, Platform};
}

pub mod store {
    pub use store::OnDisk;
}

pub use app::App;
pub use audio;
pub use dispatch::{async_after, from_main, on_main, wait_for_next_frame};
pub use manage::data_manager::DataManager;
pub use paths::*;
pub use vents::{DelayedEvent, Event, OnceEvent};
pub use wgpu_wrapper::cast_slice;
