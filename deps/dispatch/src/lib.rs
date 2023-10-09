#![feature(const_trait_impl)]
#![feature(async_fn_in_trait)]
#![feature(arbitrary_self_types)]

extern crate core;

mod dispatch;

pub use crate::dispatch::*;
