
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

#[macro_use] mod utils;
#[macro_use] extern crate guard;

mod gm;
mod te;
mod ui;
mod gl_wrapper;

use crate::gm::Size;
use crate::te::Screen;
use crate::gl_wrapper::GLDrawer;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::borrow::Borrow;
use crate::utils::{Shared, make_shared};

struct Obj {
    pub val: i8
}

struct Kok {
    pub sok: Cell<Obj>
}

fn main() {
    GLDrawer::
    <Screen>::
    with_size(Size { width: 800.0, height: 800.0 })
        .start_main_loop();
}