#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

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
use std::ops::{Deref};

struct Drawable {
    pub handler: u32
}

impl Drawable {
    pub fn draw(&self) {
        log!("kok")
    }
}

struct Square {
    pub base: Drawable,
    pub side: u32
}

impl Deref for Square {
    type Target = Drawable;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

struct Circle {
    pub base: Drawable,
    pub radius: u32
}

fn start() {}

fn main() {

    let sq = Square {
        base: Drawable {
            handler: 0
        },
        side: 0
    };


    sq.draw();

 //   return;

    GLDrawer::<Screen>::with_size(Size {
        width: 1000.0,
        height: 600.0,
    })
    .start_main_loop();
}
