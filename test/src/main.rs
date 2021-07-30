use gl_wrapper::GLDrawer;
use test_engine::TestScreen;

extern crate proc_macro;
use proc_macro::make_answer;

make_answer!();



fn main() {
    dbg!(answer());


    return;
    GLDrawer::<TestScreen>::with_size((1200, 600).into()).start_main_loop();
}
