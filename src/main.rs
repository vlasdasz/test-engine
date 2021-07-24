#![allow(dead_code)]

#[macro_use]
pub extern crate tools;
#[macro_use]
extern crate guard;

mod gm;
mod image;
mod te;
mod ui;
#[macro_use]
mod gl_wrapper;
mod sprites;

use crate::sprites::Control;
use crate::sprites::Level;
use crate::{gl_wrapper::GLDrawer, gm::Size, te::ui::TestView};
use tools::new;
use tools::refs::new_shared;

fn main() {
    let drawer = GLDrawer::with_size(Size::make(1200, 600));

    let view: TestView = new();
    let level = new_shared::<Level>();

    let a = level.clone();
    view.dpad.borrow_mut().on_up.subscribe(move |_| {
        let mut level = a.borrow_mut();
        level.jump();
    });

    drawer.with_view(view).with_level(level).start_main_loop();
}
