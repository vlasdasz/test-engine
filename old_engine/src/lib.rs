#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(let_chains)]
#![feature(arbitrary_self_types)]

pub use audio;
pub use gen;
pub use gm;
pub use manage;
pub use reflected;
pub use rest;
pub use rtools;
pub use ui;
pub use ui_views;
pub use wgpu_wrapper;

pub use crate::screen::Screen;

pub mod app;
pub mod assets;
mod keymap;
pub mod paths;
pub mod screen;
pub mod sprite_view;
mod sprites_drawer;
pub mod ui_layer;

pub use app::*;
pub use dispatch::*;
pub use keymap::*;
pub use ui::ui_proc::*;

#[cfg(mobile)]
extern crate mashup;

#[macro_use]
extern crate log;
extern crate core;
