// Rust

#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

mod gm;
mod te;
mod utils;
mod gl_wrapper;

use crate::gm::*;
use crate::gl_wrapper::gl_wrapper::GL;

fn main() {
    GL::init(Size { width: 500.0, height: 500.0 });
}