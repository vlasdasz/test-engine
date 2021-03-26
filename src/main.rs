// Rust

#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

#[macro_use] mod utils;
#[macro_use] extern crate guard;

mod ui;
mod gm;
mod te;
mod gl_wrapper;

use crate::gm::*;

use crate::te::*;
use crate::gl_wrapper::GLDrawer;


fn main() {
    GLDrawer::
    <Screen>::
    with_size(Size { width: 800.0, height: 800.0 })
        .start_main_loop();
}