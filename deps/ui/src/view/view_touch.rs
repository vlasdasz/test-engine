use crate::{
    input::UIEvents,
    view::{view_data::ViewData, view_touch_internal::ViewTouchInternal, ViewFrame},
    Touch, TouchStack, View, ViewTouchCallbacks, WeakView,
};

pub trait ViewTouch {
    fn is_selected(&self) -> bool;
    fn enable_touch(&self);
    fn enable_touch_low_priority(&self);
    fn disable_touch(&self);
    fn touch(&self) -> &ViewTouchCallbacks;
}

impl<T: ?Sized + View> ViewTouch for T {
    fn is_selected(&self) -> bool {
        self.base().is_selected
    }

    fn enable_touch(&self) {
        TouchStack::enable_for(self.weak_view(), true);
    }

    fn enable_touch_low_priority(&self) {
        TouchStack::enable_for(self.weak_view(), false);
    }

    fn disable_touch(&self) {
        TouchStack::disable_for(self.weak_view());
    }

    fn touch(&self) -> &ViewTouchCallbacks {
        &self.base().touch
    }
}

pub fn check_touch(mut view: WeakView, touch: &mut Touch) -> bool {
    if view.is_null() || view.is_hidden() {
        return false;
    }

    if touch.is_moved() && view.touch_id() == touch.id {
        touch.position -= view.absolute_frame().origin;
        view.base().touch.all.trigger(*touch);
        return true;
    }

    if touch.is_moved() {
        return false;
    }

    if touch.is_ended() && view.touch_id() == touch.id {
        let inside = view.absolute_frame().contains(touch.position);

        touch.position -= view.absolute_frame().origin;
        view.set_touch_id(0);
        view.base().touch.all.trigger(*touch);

        if inside {
            view.base().touch.up_inside.trigger(*touch);
        }
        return true;
    }

    if view.absolute_frame().contains(touch.position) {
        touch.position -= view.absolute_frame().origin;
        if touch.is_began() {
            view.set_touch_id(touch.id);
            view.base().touch.began.trigger(*touch);
            UIEvents::get().set_selected(view, true);
        }
        view.base().touch.all.trigger(*touch);
        return true;
    }

    if touch.is_began() {
        UIEvents::get().unselect_view();
    }

    false
}
