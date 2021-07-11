use std::rc::Rc;
use crate::te::Assets;

pub struct SpritesDrawer {
    assets: Rc<Assets>
}

impl SpritesDrawer {

    pub fn new(assets: Rc<Assets>) -> Self {
        Self {
            assets
        }
    }

    //pub fn draw()

}
