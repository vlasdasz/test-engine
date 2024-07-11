use refs::Weak;
use vents::Event;

pub struct KeyAction {
    pub key:    char,
    action:     Event,
    subscriber: Weak,
}

impl KeyAction {
    pub fn new<T: ?Sized>(subscriber: Weak<T>, key: char, action: impl FnMut() + Send + 'static) -> Self {
        let event = Event::default();
        event.sub(action);
        Self {
            subscriber: subscriber.erase(),
            key,
            action: event,
        }
    }
}

impl KeyAction {
    pub fn check(&self, key: char) -> bool {
        if self.subscriber.is_null() {
            return false;
        }
        if self.key == key {
            self.action.trigger(());
        }
        true
    }
}
