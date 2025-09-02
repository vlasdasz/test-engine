#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod app;
mod interface;
mod levels;
mod no_physics;

#[cfg(not(ios))]
pub use test_engine;

#[cfg(ios)]
test_engine::register_app!(crate::app::TestGameApp);
