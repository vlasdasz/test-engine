#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(trait_upcasting)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod benchmark;
mod test_game;
mod ui_test;
mod views_testing;

use test_engine::{App, MakeApp};

use crate::test_game::TestApp;

#[tokio::main]
async fn main() {
    TestApp::make_app().launch()
}
