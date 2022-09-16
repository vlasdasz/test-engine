use rtools::{static_get, Event, Rglica};

use crate::{input::keyboard::KeyEvent, View, ViewTouch};

#[derive(Default)]
pub struct UIEvents {
    pub key_pressed:   Event<KeyEvent>,
    pub selected_view: Rglica<dyn View>,
}

impl UIEvents {
    pub fn unselect_view(&mut self) {
        if let Some(view) = self.selected_view.get() {
            view.set_selected(false)
        }
    }
}

static_get!(UIEvents);
