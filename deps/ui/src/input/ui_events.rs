use rtools::{static_get, Event};

use crate::input::KeyEvent;

#[derive(Default)]
pub struct UIEvents {
    pub on_key_pressed: Event<(String, KeyEvent)>,
}

static_get!(UIEvents);
