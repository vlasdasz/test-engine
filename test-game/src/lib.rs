#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod api;
mod app;
mod interface;
mod levels;
mod no_physics;

#[cfg(not(ios))]
pub use test_engine;

#[cfg(ios)]
test_engine::register_app!(crate::app::TestGameApp);

type TestFn = fn() -> anyhow::Result<()>;

pub static UI_TESTS: parking_lot::Mutex<std::collections::BTreeMap<String, TestFn>> =
    parking_lot::Mutex::new(std::collections::BTreeMap::new());
