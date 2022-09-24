use crate::{
    input::UIEvents,
    view::{view_touch_internal::ViewTouchInternal, ViewFrame, ViewSubviews},
    Touch, View,
};

pub trait ViewTouch {
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool);
    fn enable_touch(&self);
    fn disable_touch(&self);
    fn check_touch(&mut self, touch: &mut Touch) -> bool;
}

impl<T: ?Sized + View> ViewTouch for T {
    fn is_selected(&self) -> bool {
        self.is_selected
    }

    fn set_selected(&mut self, selected: bool) {
        let mut events = UIEvents::get();

        if let Some(selected) = events.selected_view.get() {
            selected.is_selected = false;
            selected.on_selection_changed(false);
            events.selected_view.reset();
        }

        if selected {
            events.selected_view = self.rglica();
        }

        self.is_selected = selected;
        self.on_selection_changed(selected);
    }

    fn enable_touch(&self) {
        self.touch_enabled.replace(true);
    }

    fn disable_touch(&self) {
        self.touch_enabled.replace(false);
    }

    fn check_touch(&mut self, touch: &mut Touch) -> bool {
        if self.is_deleted {
            return false;
        }

        if self.touch_enabled() {
            if touch.is_moved() && self.touch_id() == touch.id {
                touch.position -= self.absolute_frame().origin;
                self.on_touch(touch);
                return true;
            }

            if touch.is_moved() {
                return false;
            }

            if touch.is_ended() && self.touch_id() == touch.id {
                touch.position -= self.absolute_frame().origin;
                self.set_touch_id(0);
                self.on_touch(touch);
                return true;
            }

            if self.absolute_frame().contains(touch.position) {
                touch.position -= self.absolute_frame().origin;
                self.set_touch_id(touch.id);
                self.on_touch(touch);
                return true;
            }
        }

        for view in self.subviews_mut().iter_mut().rev() {
            if view.check_touch(touch) {
                return true;
            }
        }

        if touch.is_began() {
            UIEvents::get().unselect_view();
        }

        false
    }
}
