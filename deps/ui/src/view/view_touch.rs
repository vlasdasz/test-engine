use crate::{
    Touch, TouchStack, UIManager, View, ViewTouchCallbacks, WeakView,
    view::{ViewFrame, view_data::ViewData, view_touch_internal::ViewTouchInternal},
};

pub trait ViewTouch {
    fn is_selected(&self) -> bool;
    fn enable_touch(&self) -> &Self;
    fn enable_touch_low_priority(&self) -> &Self;
    fn disable_touch(&self);
    fn touch(&self) -> &ViewTouchCallbacks;
}

impl<T: ?Sized + View> ViewTouch for T {
    fn is_selected(&self) -> bool {
        self.__base_view().is_selected
    }

    fn enable_touch(&self) -> &Self {
        TouchStack::enable_for(self.weak_view());
        self
    }

    fn enable_touch_low_priority(&self) -> &Self {
        TouchStack::enable_for_low_priority(self.weak_view());
        self
    }

    fn disable_touch(&self) {
        TouchStack::disable_for(self.weak_view());
    }

    fn touch(&self) -> &ViewTouchCallbacks {
        &self.__base_view().touch
    }
}

pub fn check_touch(mut view: WeakView, touch: &mut Touch) -> bool {
    if view.is_null() || view.is_hidden() {
        return false;
    }

    if touch.is_moved() && view.touch_id() == touch.id {
        touch.position -= view.absolute_frame().origin;
        view.__base_view().touch.all.trigger(*touch);
        view.__base_view().touch.moved.trigger(*touch);
        return true;
    }

    if touch.is_moved() {
        return false;
    }

    if touch.is_ended() && view.touch_id() == touch.id {
        let inside = view.absolute_frame().contains(touch.position);

        touch.position -= view.absolute_frame().origin;
        view.set_touch_id(0);
        view.__base_view().touch.all.trigger(*touch);

        if inside {
            view.__base_view().touch.up_inside.trigger(*touch);
        }
        return true;
    }

    if view.absolute_frame().contains(touch.position) {
        touch.position -= view.absolute_frame().origin;
        if touch.is_began() {
            view.set_touch_id(touch.id);
            view.__base_view().touch.began.trigger(*touch);
            UIManager::set_selected(view, true);
        }
        view.__base_view().touch.all.trigger(*touch);
        return true;
    }

    if touch.is_began() {
        UIManager::unselect_view();
    }

    false
}
