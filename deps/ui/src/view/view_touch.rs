use refs::Weak;

use crate::{
    input::UIEvents,
    view::{view_touch_internal::ViewTouchInternal, ViewFrame},
    Touch, TouchStack, View,
};

pub trait ViewTouch {
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool);
    fn enable_touch(&self);
    fn enable_touch_low_priority(&self);
    fn disable_touch(&self);
}

impl<T: ?Sized + View> ViewTouch for T {
    fn is_selected(&self) -> bool {
        self.is_selected
    }

    fn set_selected(&mut self, selected: bool) {
        let events = UIEvents::get();

        if let Some(selected) = events.selected_view.get() {
            selected.is_selected = false;
            selected.on_selection_changed(false);
            events.selected_view = Default::default();
        }

        if selected {
            events.selected_view = self.weak_view();
        }

        self.is_selected = selected;
        self.on_selection_changed(selected);
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
}

pub fn check_touch(mut view: Weak<dyn View>, touch: &mut Touch) -> bool {
    if view.freed() || view.is_hidden {
        return false;
    }

    if touch.is_moved() && view.touch_id() == touch.id {
        touch.position -= view.absolute_frame().origin;
        view.touch.all.trigger(*touch);
        return true;
    }

    if touch.is_moved() {
        return false;
    }

    if touch.is_ended() && view.touch_id() == touch.id {
        let inside = view.absolute_frame().contains(touch.position);

        touch.position -= view.absolute_frame().origin;
        view.set_touch_id(0);
        view.touch.all.trigger(*touch);

        if inside {
            view.touch.up_inside.trigger(*touch);
        }
        return true;
    }

    if view.absolute_frame().contains(touch.position) {
        touch.position -= view.absolute_frame().origin;
        if touch.is_began() {
            view.set_touch_id(touch.id);
            view.touch.began.trigger(*touch);
        }
        view.touch.all.trigger(*touch);
        return true;
    }

    if touch.is_began() {
        UIEvents::get().unselect_view();
    }

    false
}
