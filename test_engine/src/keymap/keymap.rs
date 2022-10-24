use std::cell::RefCell;

use ui::refs::Weak;

use crate::KeyAction;

#[derive(Default)]
pub struct Keymap {
    keys: RefCell<Vec<KeyAction>>,
}

impl Keymap {
    pub fn add<Obj: 'static>(&self, key: char, obj: &Obj, action: impl FnMut(Weak<Obj>) + 'static) {
        self.keys.borrow_mut().push(KeyAction::new(key, obj, action))
    }

    pub fn check(&self, key: char) {
        self.keys.borrow().iter().for_each(|a| a.check(key))
    }
}
