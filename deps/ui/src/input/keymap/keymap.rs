use std::cell::RefCell;

use crate::KeyAction;

#[derive(Default)]
pub struct Keymap {
    keys: RefCell<Vec<KeyAction>>,
}

impl Keymap {
    pub fn add(&self, key: char, action: impl FnMut() + 'static) {
        self.keys.borrow_mut().push(KeyAction::new(key, action))
    }

    pub fn check(&self, key: char) {
        self.keys.borrow().iter().for_each(|a| a.check(key))
    }
}
