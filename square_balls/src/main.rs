use gl_wrapper::Screen;
use test_engine::TestScreen;
use tools::has_new::New;

fn main() {
    TestScreen::new()
        .set_size((1000, 800).into())
        .start_main_loop();
}
