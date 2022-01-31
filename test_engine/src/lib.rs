#![allow(incomplete_features)]
#![feature(trait_upcasting)]

pub use gl_image;
pub use gl_wrapper;
pub use gm;
pub use maze;
pub use rtools;
pub use sprites;
pub use ui;

pub use crate::screen::Screen;

pub mod assets;
pub mod debug_view;
pub mod game_view;
pub mod paths;
pub mod screen;
mod sprites_drawer;
mod ui_drawer;
pub mod ui_layer;

pub use gl_image::Image;
pub use sprites::{Level, LevelBase, Sprite};

#[cfg(any(target_os = "ios", target_os = "android"))]
#[macro_use]
extern crate mashup;

#[macro_use]
extern crate log;
