// Rust

#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

#[macro_use] extern crate guard;
#[macro_use] mod utils;

mod gm;
mod te;
mod gl_wrapper;

use crate::gm::*;
use crate::gl_wrapper::gl_wrapper::GLWrapper;

fn main() {
    GLWrapper::init(Size { width: 500.0, height: 500.0 });
}