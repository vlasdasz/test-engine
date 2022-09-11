use rtools::Event;

use crate::{
    view::{view_touch_internal::ViewTouchInternal, ViewFrame, ViewSubviews},
    Touch, View,
};

pub trait ViewTouch {
    fn on_touch(&self) -> &Event<Touch>;
    fn check_touch(&mut self, touch: &mut Touch) -> bool;
}

impl<T: ?Sized + View> ViewTouch for T {
    fn on_touch(&self) -> &Event<Touch> {
        self.touch_enabled.replace(true);
        &self.on_touch
    }

    fn check_touch(&mut self, touch: &mut Touch) -> bool {
        if self.touch_enabled() {
            if touch.is_moved() && self.touch_id() == touch.id {
                touch.position -= self.absolute_frame().origin;
                self.on_touch().trigger(*touch);
                return true;
            }

            if touch.is_moved() {
                return false;
            }

            if touch.is_ended() && self.touch_id() == touch.id {
                touch.position -= self.absolute_frame().origin;
                self.set_touch_id(0);
                self.on_touch().trigger(*touch);
                return true;
            }

            if self.absolute_frame().contains(touch.position) {
                touch.position -= self.absolute_frame().origin;
                self.set_touch_id(touch.id);
                self.on_touch().trigger(*touch);
                return true;
            }
        }

        for view in self.subviews_mut().iter_mut().rev() {
            if view.check_touch(touch) {
                return true;
            }
        }

        false
    }
}
