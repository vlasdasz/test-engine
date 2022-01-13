mod test_game_level;
mod test_game_view;

use test_engine::Screen;
use tools::Boxed;

use crate::test_game_view::TestGameView;

fn main() {

    let five = 5;

    println!("Hello: {five}");

    let mut screen = Screen::new((1000, 600).into());

    screen.ui.set_view(TestGameView::boxed());
    screen.ui.add_debug_view();

    screen.start_main_loop();
}
