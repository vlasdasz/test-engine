#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod app;
mod interface;
mod levels;
mod no_physics;

pub use test_engine;

use crate::app::TestGameApp;

test_engine::register_app!(TestGameApp);
