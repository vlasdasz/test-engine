#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]
#![feature(linkage)]

mod interface;
mod levels;
mod no_physics;

pub use test_game::test_engine_create_app;

fn main() {
    test_engine::test_engine_start_app();
}
