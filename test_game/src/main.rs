mod test_level;

use test_engine::Screen;
use test_level::TestLevel;
use tools::Boxed;

fn main() {
    Screen::new((1000, 800).into())
        .set_level(TestLevel::boxed())
        .start_main_loop();
}
