#![allow(dead_code)]

#[macro_use]
pub extern crate tools;
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

fn main() {
    GLDrawer::with_size((1200, 600).into()).start_main_loop();
}
