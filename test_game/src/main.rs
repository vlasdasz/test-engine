mod test_level;

use test_engine::Screen;

fn main() { Screen::new((1000, 800).into()).start_main_loop(); }
