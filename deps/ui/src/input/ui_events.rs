use rtools::{static_get, Event};

use crate::input::KeyEvent;

#[derive(Default)]
pub struct UIEvents {
    pub key_pressed: Event<(String, KeyEvent)>,
}

static_get!(UIEvents);
