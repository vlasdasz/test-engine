#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(linkage)]

use futures::future::BoxFuture;
use parking_lot::Mutex;

use crate::app::TestGameApp;

mod api;
mod app;
mod interface;
mod levels;
mod no_physics;

type AsyncFn = fn() -> BoxFuture<'static, anyhow::Result<()>>;
pub static UI_TESTS: Mutex<Vec<AsyncFn>> = Mutex::new(Vec::new());

test_engine::register_app!(TestGameApp);

fn main() {
    test_engine::test_engine_start_app();
}
