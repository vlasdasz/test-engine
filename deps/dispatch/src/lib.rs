#![feature(const_trait_impl)]
#![feature(arbitrary_self_types)]

extern crate core;

mod dispatch;
mod task;

pub use task::Task;

pub use crate::dispatch::*;
