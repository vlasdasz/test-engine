use gl_wrapper::GLDrawer;
use test_engine::TestScreen;

extern crate proc_macro;
extern crate tools;
use proc_macro::AsAny;

#[derive(AsAny)]
struct Spesogon {}

fn main() {
    let spes = Spesogon {};

    drop(spes);

    //return;
    GLDrawer::<TestScreen>::with_size((1200, 600).into()).start_main_loop();
}
