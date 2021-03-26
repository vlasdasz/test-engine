use crate::gl_wrapper::Updatable;
use crate::gm::Size;

pub struct Screen {
    window_size: Size
}

impl Screen {

}

impl Updatable for Screen {
    fn new() -> Screen {
        Screen { window_size: Size::new() }
    }
    fn update(&mut self, windows_size: &Size) {
        self.window_size = *windows_size
    }
}
