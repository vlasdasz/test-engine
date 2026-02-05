#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use test_engine::App;

use crate::app::InspectorApp;

mod app;
mod ui;

fn main() {
    InspectorApp::start();
}
