use vents::Event;

use crate::WeakView;

pub struct KeyAction {
    pub key: char,
    action:  Event,
    view:    WeakView,
}

impl KeyAction {
    pub fn new(view: WeakView, key: char, action: impl FnMut() + 'static) -> Self {
        let event = Event::default();
        event.sub(action);
        Self {
            view,
            key,
            action: event,
        }
    }
}

impl KeyAction {
    pub fn check(&self, key: char) -> bool {
        if self.view.is_null() {
            return false;
        }
        if self.key == key {
            self.action.trigger(())
        }
        true
    }
}
