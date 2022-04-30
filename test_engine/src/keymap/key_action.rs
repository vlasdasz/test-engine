use rtools::Event;

pub struct KeyAction {
    pub key: String,
    action:  Event,
}

impl KeyAction {
    pub fn new<Obj: 'static>(key: &str, obj: &Obj, mut action: impl FnMut(&mut Obj) + 'static) -> Self {
        let event = Event::default();
        event.set(obj, move |obj, _| action(obj));
        Self {
            key:    key.into(),
            action: event,
        }
    }
}

impl KeyAction {
    pub fn check(&self, key: &str) {
        if self.key == key {
            self.action.trigger(())
        }
    }
}
