use gl_wrapper::Screen;
use test_engine::TestScreen;

use crate::tools::New;

extern crate proc_macro;
extern crate tools;

fn main() {
    let mut screen = TestScreen::new();
    screen.init();
    screen.start_main_loop();
}
