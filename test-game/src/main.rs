#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod interface;
mod levels;

fn main() {
    #[cfg(desktop)]
    test_game::start_test_game()
}
