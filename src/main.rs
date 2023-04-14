#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(trait_upcasting)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod benchmark;
mod test_game;
mod ui_test;

use test_engine::{App, MakeApp, ViewApp};
use ui::Container;

use crate::test_game::TestApp;

#[tokio::main]
async fn main() {
    ViewApp::<Container>::start();

    //  TestApp::make_app().launch()
}
