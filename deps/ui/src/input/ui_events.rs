use refs::Weak;
use rtools::{static_default};
use vents::Event;

use crate::{input::keyboard::KeyEvent, View, ViewTouch};

#[derive(Default)]
pub struct UIEvents {
    pub key_pressed:   Event<KeyEvent>,
    pub selected_view: Weak<dyn View>,
}
static_default!(UIEvents);

impl UIEvents {
    pub fn unselect_view(&mut self) {
        if let Some(view) = self.selected_view.get() {
            view.set_selected(false)
        }
    }
}
