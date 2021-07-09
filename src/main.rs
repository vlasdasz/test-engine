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
use crate::te::ui::TestModel;
use crate::te::Screen;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use tools::refs::make_shared;

struct TestData {
    pub i: u32
}

impl TestData {
    pub fn test(&mut self) {
        self.i += 1;
        dbg!(self.i);
    }

    // pub fn hoh(self: Rc<Cell<Self>>) {
    //
    // }
}

fn main() {

    let koke = make_shared(TestData { i: 10 });

    koke.borrow_mut().test();

    return;
    GLDrawer::<Screen<TestModel>>::with_size(Size::make(800, 600)).start_main_loop();
}
