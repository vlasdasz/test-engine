use refs::Weak;

use crate::{
    input::UIEvents,
    view::{view_touch_internal::ViewTouchInternal, ViewFrame},
    Touch, UIManager, View,
};

pub trait ViewTouch {
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool);
    fn enable_touch(&self);
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
        UIManager::enable_touch_for(self.weak_view());
    }

    fn disable_touch(&self) {
        UIManager::disable_touch_for(self.weak_view());
    }
}

pub fn check_touch(mut view: Weak<dyn View>, touch: &mut Touch, skip_select: bool) -> bool {
    if view.freed() || view.is_hidden || view.is_deleted {
        return false;
    }

    if touch.is_moved() && view.touch_id() == touch.id {
        touch.position -= view.absolute_frame().origin;
        view.on_touch.trigger(*touch);
        return true;
    }

    if touch.is_moved() {
        return false;
    }

    if touch.is_ended() && view.touch_id() == touch.id {
        touch.position -= view.absolute_frame().origin;
        view.set_touch_id(0);
        view.on_touch.trigger(*touch);
        return true;
    }

    if view.absolute_frame().contains(touch.position) {
        touch.position -= view.absolute_frame().origin;
        view.set_touch_id(touch.id);
        view.on_touch.trigger(*touch);
        if touch.is_began() {
            view.on_touch_began.trigger(*touch);
        }
        return true;
    }

    if touch.is_began() && !skip_select {
        UIEvents::get().unselect_view();
    }

    false
}
