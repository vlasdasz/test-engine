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
use std::pin::Pin;
use std::ops::Deref;
use std::ptr::{null};


#[derive(Debug)]
struct View {
    pub super_view: *const View,
    pub subviews: Vec<View>,
}

impl View {

    pub fn new() -> View {
        View {
            super_view: null(),
            subviews: vec![]
        }
    }

    pub fn get_super_view(&self) -> Option<&View> {
        if self.super_view.is_null() {
            return None;
        }
        return Some(unsafe { &*self.super_view });
    }

    pub fn add_subview(&mut self, mut view: View) {
        view.super_view = self as *const View;
        self.subviews.push(view)
    }

}


fn main() {

    let mut view = View::new();

    let subview = View::new();

    view.add_subview(subview);


    dbg!(&view);


    GLDrawer::<Screen>::with_size(Size {
        width: 1000.0,
        height: 600.0,
    })
    .start_main_loop();
}
