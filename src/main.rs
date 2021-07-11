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

fn main() {
    GLDrawer::<Screen<TestModel>>::with_size(Size::make(1200, 600)).start_main_loop();
}
