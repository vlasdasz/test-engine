
use std::rc::Rc;

use crate::gm::Size;

use crate::te::{Assets, TEUIDrawer};
use crate::gl_wrapper::GLLoader;
use std::cell::RefCell;
use crate::ui::View;
use crate::gl_wrapper::gl_loader::Updatable;

pub struct Screen<'a> {
    pub gl_loader: &'a GLLoader,
    pub assets: Assets
}

impl<'a> Screen<'a> {

}

impl<'a> Updatable for Screen<'a> {
    fn update(&mut self) {

    }
}
