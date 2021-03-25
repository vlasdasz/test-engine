
use std::rc::Rc;

use crate::gm::Size;

use crate::te::{Assets, TEUIDrawer};
use crate::gl_wrapper::GLWrapper;
use std::cell::RefCell;
use crate::ui::View;

pub struct ScreenBase {
    pub root_view: View,
    gl_wrapper: Rc<RefCell<GLWrapper>>,
    assets:     Rc<Assets>,
    ui_drawer: TEUIDrawer,
}

impl ScreenBase {
    pub fn with_size(size: Size) -> ScreenBase {
        // let kok = Box::new(||{
        //     log!(&self.root_view.color);
        // });
        let gl_wrapper = Rc::new(RefCell::new(GLWrapper::with_size(size)));
        let assets     = Rc::new(Assets::init());
        let ui_drawer = TEUIDrawer::new(&assets, &gl_wrapper);
        ScreenBase { root_view: View::new(), gl_wrapper, assets, ui_drawer }
    }
    pub fn start_main_loop(&self, frame: fn () -> ()) {
        self.gl_wrapper.borrow_mut().start_main_loop();
    }
}

pub trait Screen<T> {
    fn with_size(size: Size) -> T;
}