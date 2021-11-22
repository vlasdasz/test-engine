#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(default_free_fn)]

pub use gl_image;
pub use gl_wrapper;
pub use gm;
pub use sprites;
pub use tools;
pub use ui;

pub use crate::screen::Screen;

mod assets;
pub mod debug_view;
pub mod paths;
pub mod screen;
mod sprites_drawer;
pub mod ui_drawer;

pub use gl_image::Image;
pub use sprites::{Level, LevelBase, Sprite};

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_use]
extern crate mashup;

#[macro_use]
extern crate log;
