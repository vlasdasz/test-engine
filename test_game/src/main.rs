#![allow(incomplete_features)]
#![feature(explicit_generic_args_with_impl_trait)]

use rtools::Boxed;
use test_engine::Screen;

#[allow(unused_imports)]
use crate::benchmark::BenchmarkView;
#[allow(unused_imports)]
use crate::test_game::TestGameView;

mod benchmark;
mod test_game;

fn main() {
    let mut screen = Screen::new((1000, 600).into());

    screen.ui.set_view(TestGameView::boxed());
    screen.ui.add_debug_view();

    screen.start_main_loop();
}
