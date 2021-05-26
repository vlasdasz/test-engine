#![allow(dead_code)]

#[macro_use]
extern crate tools;
#[macro_use]
extern crate guard;
#[macro_use]
extern crate derivative;

mod gm;
mod image;
mod te;
mod ui;
#[macro_use]
mod gl_wrapper;

use crate::gl_wrapper::GLDrawer;
use crate::gm::Size;
use crate::te::Screen;
use std::fmt::Debug;
use tools::HasNew;

trait Object where Self: Debug {
    fn get_obj(&self) -> &BaseObject;
}

#[derive(Debug)]
struct BaseObject {
    pub i: u32,
    pub sub: Vec<Box<dyn Object>>
}

impl HasNew for BaseObject {
    fn new() -> BaseObject {
        BaseObject {
            i: 15,
            sub: vec![]
        }
    }
}

impl Object for BaseObject {
    fn get_obj(&self) -> &BaseObject {
        &self
    }
}


fn main() {
    GLDrawer::<Screen>::with_size(Size {
        width: 1000.0,
        height: 600.0,
    })
    .start_main_loop();
}
