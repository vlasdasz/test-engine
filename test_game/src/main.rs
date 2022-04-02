use rtools::Boxed;
use test_engine::Screen;

use crate::benchmark::BenchmarkView;

mod benchmark;
mod test_game;

fn main() {
    let mut screen = Screen::new((1000, 600).into());

    screen.ui.set_view(BenchmarkView::boxed());
    screen.ui.add_debug_view();

    screen.start_main_loop();
}
