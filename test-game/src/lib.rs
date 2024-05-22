#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod interface;
mod levels;
pub mod test_game;

#[cfg(mobile)]
#[no_mangle]
extern "C" fn test_game() -> std::ffi::c_int {
    test_game::start_test_game();
    0
}
