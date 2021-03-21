// Rust

#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

extern crate rustc_serialize;
use rustc_serialize::json;

mod gm;
mod gl_wrapper;
mod te;
mod utils;

use utils::log;
use gm::Point;
use gl_wrapper::GL;
use crate::gm::{Size, Color};


fn main() {
    let point = Point { x: 5.0, y: 5.0 };
    let new_point = Point::new();

    println!(
        "The area of the rectangle is {} square pixels. A {}",
        point.to_string(),
        new_point.to_string()
    );

    log(&point.normalized().length());

    let encoded = json::encode(&point).unwrap();

    log(&encoded);

    log(&te::paths::shaders::isometric().to_string_lossy());

    log(&Color::random().to_string());

    GL::init(Size { width: 500.0, height: 500.0 });

}