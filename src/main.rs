#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(trait_upcasting)]
#![feature(arbitrary_self_types)]

mod benchmark;
mod test_game;
mod ui_test;

use crate::test_game::TestApp;

#[tokio::main]
async fn main() {
    TestApp::default().launch()
}
