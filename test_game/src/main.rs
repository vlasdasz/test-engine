mod test_level;
mod test_view;

use test_engine::Screen;
use tools::Boxed;

use crate::test_view::TestGameView;

fn main() {
    let mut screen = Screen::new((1000, 600).into());

    screen.ui.set_view(TestGameView::boxed());
    screen.ui.add_debug_view();

    screen.start_main_loop();
}
