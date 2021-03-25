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

use crate::te::*;
use crate::gl_wrapper::{GLLoader, GLDrawer};


struct Kok {
    pub val: i8
}

struct KokObserver<'a> {
    kok: &'a Kok
}

struct KokUser<'a> {
    kok: &'a mut Kok
}

impl<'a> KokUser<'a> {
    pub fn use_kok(&mut self) {
        self.kok.val += 1;
        log!(self.kok.val)
    }
}

fn main() {

    let mut kok = Kok { val: 10 };

    let kok_observer = KokObserver { kok: &kok };
    let mut kok_user = KokUser { kok: &mut kok };

    kok_user.use_kok();

    // let mut gl_loader = GLLoader::with_size(Size { width: 500.0, height: 500.0 });
    //
    // let assets = Assets::init();
    //
    // let screen = Screen { gl_loader: &gl_loader, assets };
    //
    // let mut gl_drawer = GLDrawer::new(&mut gl_loader, screen);
    //
    //
    // gl_drawer.start_main_loop();

}