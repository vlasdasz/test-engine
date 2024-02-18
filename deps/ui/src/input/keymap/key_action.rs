use vents::Event;

pub struct KeyAction {
    pub key: char,
    action:  Event,
}

impl KeyAction {
    pub fn new(key: char, action: impl FnMut() + 'static) -> Self {
        let event = Event::default();
        event.sub(action);
        Self { key, action: event }
    }
}

impl KeyAction {
    pub fn check(&self, key: char) {
        if self.key == key {
            self.action.trigger(())
        }
    }
}
