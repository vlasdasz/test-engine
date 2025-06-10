#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod app;
mod interface;

use crate::app::DebugApp;

test_engine::register_app!(DebugApp);
