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

use crate::gl_wrapper::GLDrawer;
use crate::gm::Size;
use crate::te::Screen;
use tools::HasNew;

struct TestModel {
    pub data: u32
}

impl HasNew for TestModel {
    fn new() -> Self {
        TestModel { data: 0 }
    }
}

fn main() {
    GLDrawer::<Screen<TestModel>>::with_size(Size::make(800, 600)).start_main_loop();
}
