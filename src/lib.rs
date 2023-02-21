#![cfg(mobile)]
#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(trait_upcasting)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

use std::os::raw::{c_float, c_int};

use test_engine::{App, MakeApp};

use crate::test_game::TestApp;

mod benchmark;
mod test_game;
mod ui_test;

#[no_mangle]
extern "C" fn make_app(
    ppi: c_int,
    scale: c_float,
    refresh_rate: c_int,
    resolution_x: c_int,
    resolution_y: c_int,
    width: c_float,
    height: c_float,
    diagonal: c_float,
) -> Box<dyn App> {
    TestApp::make_app(
        ppi,
        scale,
        refresh_rate,
        resolution_x,
        resolution_y,
        width,
        height,
        diagonal,
    )
}
