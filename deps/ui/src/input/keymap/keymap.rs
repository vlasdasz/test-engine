use std::cell::RefCell;

use crate::{KeyAction, WeakView};

#[derive(Default)]
pub struct Keymap {
    keys: RefCell<Vec<KeyAction>>,
}

impl Keymap {
    pub fn add(&self, view: WeakView, key: char, action: impl FnMut() + 'static) {
        self.keys.borrow_mut().push(KeyAction::new(view, key, action))
    }

    pub fn check(&self, key: char) {
        self.keys.borrow_mut().retain(|a| a.check(key));
    }
}
