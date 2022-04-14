#![allow(incomplete_features)]
#![feature(explicit_generic_args_with_impl_trait)]

use test_engine::{paths::home, rtools::Boxed, Screen};

#[macro_use]
extern crate log;

#[allow(unused_imports)]
use crate::benchmark::BenchmarkView;
#[allow(unused_imports)]
use crate::test_game::TestGameView;

mod benchmark;
mod test_game;

fn main() {
    let mut screen = Screen::new(&home().join("test_engine"), (1000, 600).into());

    screen.ui.set_view(TestGameView::boxed());
    screen.ui.add_debug_view();

    screen.start_main_loop();
}
