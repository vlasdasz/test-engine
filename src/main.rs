#![allow(incomplete_features)]
#![allow(clippy::module_inception)]
#![feature(specialization)]
#![feature(trait_upcasting)]

use test_engine::{
    paths::home,
    rtools::{init_log, Boxed},
    Screen,
};

use crate::{
    benchmark::BenchmarkView,
    test_game::{TestGameLevel, TestGameView},
};

mod benchmark;
mod test_game;
mod ui_test;

#[tokio::main]
async fn main() {
    init_log(false, 4);

    let mut screen = Screen::new((1000, 600), &home().join("test_engine"), TestGameView::boxed());

    screen.ui.set_level(TestGameLevel::boxed());

    screen.start_main_loop();
}
