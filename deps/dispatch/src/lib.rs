#![feature(const_trait_impl)]
#![feature(arbitrary_self_types)]

extern crate core;

mod dispatch;
mod spawn;
mod task;
// pub use task::Task;

pub use dispatch::*;
pub use spawn::*;
