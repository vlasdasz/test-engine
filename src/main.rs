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
use crate::gl_wrapper::GLWrapper;

struct Parent<'a> {
    pub parent: Option<&'a Parent<'a>>,
    pub childred: Vec<Parent<'a >>
}


impl<'a> Parent<'a> {

    pub fn new() -> Parent<'a> {
        Parent { parent: None, childred: vec![] }
    }

    pub fn add_child(&mut self, child: Parent) {
        // let mut ch = Parent { parent: None, childred: vec![] };
        // ch.parent = Some(self);
        // self.childred.push(ch)
    }
}

fn main() {

    // let mut items = vec![1];
    // let mut item = items.last();
    //
    // items.push(2);
    //
    // log!(&mut item);


    // let mut root = Parent::new();
    //
    // let mut child = Parent::new();
    //
    // child.parent = Some(&root);
    // root.childred.push(child);


    GLWrapper::init(Size { width: 500.0, height: 500.0 });
}