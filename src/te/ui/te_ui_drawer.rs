
use std::rc::Rc;

use crate::ui::View;
use crate::te::Assets;
use crate::gl_wrapper::GLLoader;

use crate::gm::{ Rect, Color };
use std::cell::RefCell;

pub struct TEUIDrawer {
    //assets:     Rc<Assets>,
    //gl_wrapper: Rc<RefCell<GLWrapper>>
}

impl TEUIDrawer {
    // pub fn new(assets: &Rc<Assets>, gl_wrapper: &Rc<RefCell<GLWrapper>>) -> TEUIDrawer {
    //     TEUIDrawer {
    //         assets:     Rc::clone(assets),
    //         gl_wrapper: Rc::clone(gl_wrapper)
    //     }
    // }
}

impl TEUIDrawer {
    pub fn draw_view(&self, view: &mut View) {
        view.calculate_absolute_frame();
        self.draw_rect(view.absolute_frame(), &view.color);
        for view in view.subviews() {
            self.draw_view(view)
        }
    }
}

impl TEUIDrawer {
    fn set_viewport(&self, rect: &Rect) {
        //self.gl_wrapper.borrow_mut().set_viewport(rect)
    }
}

impl TEUIDrawer {
    fn fill_rect(&self, rect: &Rect, color: &Color) {
        // self.set_viewport(rect);
        // self.assets.shaders.ui.enable();
        // self.assets.shaders.ui.set_color(color);
        // self.assets.buffers.fullscreen.draw();
    }
    fn draw_rect(&self, rect: &Rect, color: &Color) {
        // self.set_viewport(rect);
        // self.assets.shaders.ui.enable();
        // self.assets.shaders.ui.set_color(color);
        // self.assets.buffers.fullscreen_outline.draw();
    }
}