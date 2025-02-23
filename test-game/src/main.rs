#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]
#![feature(linkage)]

mod interface;
mod levels;
mod no_physics;

fn main() {
    test_engine::launch_app!();
}
