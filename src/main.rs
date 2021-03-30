
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

#[macro_use] mod utils;
#[macro_use] extern crate guard;

mod gm;
mod te;
mod ui;
mod image;
mod gl_wrapper;

use crate::gm::Size;
use crate::te::Screen;
use crate::gl_wrapper::GLDrawer;


fn main() {
    GLDrawer::
    <Screen>::
    with_size(Size { width: 1000.0, height: 600.0 })
        .start_main_loop();
}