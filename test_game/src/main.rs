mod test_level;
mod test_view;

use rtools::Boxed;
use test_engine::Screen;

use crate::test_view::TestView;

fn main() {
    let mut screen = Screen::new((1000, 600).into());

    screen.ui.set_view(TestView::boxed());
    screen.ui.add_debug_view();

    screen.start_main_loop();
}
