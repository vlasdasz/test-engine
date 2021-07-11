#![allow(dead_code)]

#[macro_use]
extern crate tools;
#[macro_use]
extern crate guard;

mod gm;
mod image;
mod te;
mod ui;
#[macro_use]
mod gl_wrapper;
mod sprites;

use crate::gl_wrapper::GLDrawer;
use crate::gm::Size;

fn main() {
    GLDrawer::with_size(Size::make(1200, 600)).start_main_loop();
}
