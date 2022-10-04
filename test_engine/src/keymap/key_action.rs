use ui::Event;

pub struct KeyAction {
    pub key: char,
    action:  Event,
}

impl KeyAction {
    pub fn new<Obj: 'static>(key: char, obj: &Obj, mut action: impl FnMut(&mut
    Obj) + 'static) -> Self {     let event = Event::default();
        event.set(obj, move |obj, _| action(obj));
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
