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
use serde::{Deserialize, Serialize};
use tools::{HasNew, PropertyWrapper};

#[derive(Serialize, Deserialize, Debug)]
struct TestGest {
    pub i: u32,
    pub stro: String,
}

impl HasNew for TestGest {
    fn new() -> TestGest {
        TestGest {
            i: 100100,
            stro: "rglo".into(),
        }
    }
}

fn main() {
    let mut sokol = PropertyWrapper::<TestGest>::new("sokol");

    dbg!(&sokol.i);
    dbg!(&sokol.stro);

    sokol.i += 1;
    sokol.stro = "guga".into();

    sokol.store();

    return;
    GLDrawer::with_size(Size::make(1200, 600)).start_main_loop();
}
