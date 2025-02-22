#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod app;
mod interface;
mod levels;
mod no_physics;

use test_engine::App;
pub use test_engine::{self};

use crate::app::TestGameApp;

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn test_engine_create_app() -> Box<dyn App> {
    Box::new(TestGameApp)
}
