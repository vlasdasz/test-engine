#![feature(let_chains)]

mod app;
mod assets;
mod paths;

pub mod ui {
    pub use ::ui::*;
    pub use gm::{
        flat::{IntSize, Points},
        Color,
    };
    pub use ui_proc::view;
    pub use ui_views::*;
    pub use wgpu_wrapper::image::Image;
}

pub use app::App;
pub use audio;
pub use manage::data_manager::DataManager;
pub use paths::*;
pub use refs;
pub use rtools::Apply;
