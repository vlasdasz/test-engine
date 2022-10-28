#![feature(const_default_impls)]
#![feature(const_trait_impl)]

extern crate core;

mod dispatch;
mod main_lock;
mod spinlock;

pub use main_lock::MainLock;
pub(crate) use spinlock::SpinLock;

pub use crate::dispatch::Dispatch;
