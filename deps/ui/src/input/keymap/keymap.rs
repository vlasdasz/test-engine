use std::cell::RefCell;

use refs::Weak;

use crate::KeyAction;

#[derive(Default)]
pub struct Keymap {
    keys: RefCell<Vec<KeyAction>>,
}

impl Keymap {
    pub fn add<T: ?Sized>(&self, subscriber: Weak<T>, key: char, action: impl FnMut() + Send + 'static) {
        self.keys.borrow_mut().push(KeyAction::new(subscriber, key, action));
    }

    pub fn check(&self, key: char) {
        self.keys.borrow_mut().retain(|a| a.check(key));
    }
}
