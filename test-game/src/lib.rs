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

pub static UI_TESTS: test_engine::__internal_macro_deps::Mutex<
    std::collections::BTreeMap<String, fn() -> anyhow::Result<()>>,
> = test_engine::__internal_macro_deps::Mutex::new(std::collections::BTreeMap::new());
