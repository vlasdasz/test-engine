mod test_level;
mod test_view;

use test_engine::Screen;
use tools::Boxed;

use crate::test_view::TestGameView;

fn main() {
    Screen::new((1000, 600).into())
        .set_view(TestGameView::boxed())
        .add_debug_view()
        .start_main_loop();
}
