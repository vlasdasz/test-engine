#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(let_chains)]
#![feature(arbitrary_self_types)]
#![feature(const_default_impls)]

pub use audio;
pub use gl_image;
pub use gl_wrapper;
pub use gm;
pub use maze;
pub use net;
pub use reflected;
pub use rtools;
pub use sprites;
pub use text;
pub use ui;

pub use crate::screen::Screen;

pub mod app;
pub mod assets;
mod keymap;
pub mod paths;
pub mod screen;
pub mod sprite_view;
mod sprites_drawer;
mod ui_drawer;
pub mod ui_layer;

pub use dispatch::*;
pub use gl_image::Image;
pub use keymap::*;
pub use sprites::{Level, LevelBase, Sprite};
pub use ui::ui_proc::*;

#[cfg(mobile)]
#[macro_use]
extern crate mashup;

#[macro_use]
extern crate log;
extern crate core;
