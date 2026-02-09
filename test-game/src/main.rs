#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(linkage)]

use crate::app::TestGameApp;

mod api;
mod app;
mod interface;
mod levels;
mod no_physics;

type AsyncFn = fn() -> futures::future::BoxFuture<'static, anyhow::Result<()>>;
pub static UI_TESTS: parking_lot::Mutex<std::collections::BTreeMap<String, AsyncFn>> =
    parking_lot::Mutex::new(std::collections::BTreeMap::new());

test_engine::register_app!(TestGameApp);

fn main() {
    test_engine::test_engine_start_app();
}
