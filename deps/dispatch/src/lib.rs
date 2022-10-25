#![feature(const_default_impls)]
#![feature(const_trait_impl)]

extern crate core;

mod dispatch;
mod main_lock;
mod spinlock;

pub use dispatch::Dispatch;
pub use main_lock::MainLock;
pub(crate) use spinlock::SpinLock;
