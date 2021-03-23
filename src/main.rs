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
use crate::gl_wrapper::GLWrapper;

struct Kok {
    pub superkok: *const Kok,
    pub subkoks: Vec<Kok>
}

impl Kok {
    pub fn new() -> Kok {
        Kok { superkok: std::ptr::null(), subkoks: vec!() }
    }
    pub fn add_kok(&mut self, mut kok: Kok) {
        kok.superkok = self;
        self.subkoks.push(kok);
    }
    pub fn get_superkok(&self) -> Option<&Kok> {
        unsafe { Some(&*self.superkok) }
    }
}

fn main() {

    let mut root_kok = Kok::new();

    let mut small_kok = Kok::new();

    small_kok.superkok = &root_kok;

    root_kok.add_kok(small_kok);
    root_kok.add_kok(Kok::new());
    root_kok.add_kok(Kok::new());






    return;

    GLWrapper::init(Size { width: 500.0, height: 500.0 });
}