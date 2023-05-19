#![feature(const_trait_impl)]
#![feature(async_fn_in_trait)]
#![feature(arbitrary_self_types)]

extern crate core;

mod dispatch;
mod dispatchable;

pub use dispatch::*;
pub use dispatchable::*;
