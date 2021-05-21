#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

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

use tools::refs::make_shared;

trait Kokable: Debug {
    fn kok(&self);
}

#[derive(Debug)]
struct Square {
    pub side: u32,
}

impl Kokable for Square {
    fn kok(&self) {
        dbg!("square kok");
    }
}

#[derive(Debug)]
struct Circle {
    pub radius: u32,
}

impl Kokable for Circle {
    fn kok(&self) {
        dbg!("circle kok");
    }
}

fn draw(dr: &dyn Kokable) {
    dr.kok();
    dbg!(dr);
}

fn main() {
    let mut vector: Vec<Box<dyn Kokable>> = vec![];

    vector.push(Box::new(Square { side: 0 }));

    vector.push(Box::new(Circle { radius: 0 }));

    let shared = make_shared(Circle { radius: 14 });

    let circle: &Circle = &shared.try_borrow().unwrap();

    draw(circle);

    GLDrawer::<Screen>::with_size(Size {
        width: 1000.0,
        height: 600.0,
    })
    .start_main_loop();
}
