use gl_wrapper::Screen;
use test_engine::TestScreen;

use crate::tools::New;

extern crate proc_macro;
extern crate tools;

fn main() {
    TestScreen::new()
        .set_size((1000, 800).into())
        .start_main_loop();
}
