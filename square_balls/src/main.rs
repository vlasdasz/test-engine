use test_engine::Screen;
use tools::has_new::New;

fn main() {
    Screen::new().set_size((1000, 800).into()).start_main_loop();
}
