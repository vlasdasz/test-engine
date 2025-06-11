#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]
#![feature(linkage)]

mod app;
mod interface;

fn main() {
    test_engine::launch_app!();
}
