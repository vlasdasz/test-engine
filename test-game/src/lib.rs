#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod api;
mod app;
mod interface;
mod levels;
mod no_physics;

use futures::future::BoxFuture;
use parking_lot::Mutex;
#[cfg(not(ios))]
pub use test_engine;

#[cfg(ios)]
test_engine::register_app!(crate::app::TestGameApp);

type AsyncFn = fn() -> BoxFuture<'static, anyhow::Result<()>>;

#[derive(Clone)]
pub struct UITestInfo {
    pub name: String,
    pub test: AsyncFn,
}

pub static UI_TESTS: Mutex<Vec<UITestInfo>> = Mutex::new(Vec::new());
