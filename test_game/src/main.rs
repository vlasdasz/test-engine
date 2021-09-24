mod test_level;
mod test_view;

use test_engine::Screen;
use tools::Boxed;

use crate::test_view::TestView;

fn main() {
    Screen::new((1000, 600).into())
        .add_view(TestView::boxed())
        .add_debug_view()
        .start_main_loop();
}
