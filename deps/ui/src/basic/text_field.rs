use gm::Color;
use rtools::{Rglica, ToRglica};

use crate::{
    input::{ControlButton, KeyboardButton, UIEvents},
    view,
    view::{ViewData, ViewSubviews, ViewTouch},
    Label, SubView, Touch, View, ViewBase, ViewCallbacks, ViewLayout,
};

#[view]
#[derive(Default)]
pub struct TextField {
    label: SubView<Label>,
}

impl TextField {
    pub fn text(&self) -> &str {
        self.label.text()
    }
}

impl ViewCallbacks for TextField {
    fn setup(&mut self) {
        self.enable_touch();
        self.set_color(Color::LIGHT_GRAY);
        self.label.place().as_background();
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_began() {
            self.set_selected(true);
        }
    }

    fn on_selection_changed(&mut self, selected: bool) {
        if selected {
            UIEvents::get().key_pressed.set(self, |this, key| {
                if this.is_selected() {
                    match key.button {
                        KeyboardButton::Letter(char) => {
                            this.label.append_text(char);
                        }
                        KeyboardButton::Control(control) => {
                            if let ControlButton::Backspace = control {
                                this.label.pop_letter();
                            }
                        }
                    };
                }
            });
        } else {
            UIEvents::get().key_pressed.unsubscribe()
        }

        self.set_color(if selected { Color::GRAY } else { Color::LIGHT_GRAY });
    }
}

impl Drop for TextField {
    fn drop(&mut self) {
        UIEvents::get().key_pressed.unsubscribe()
    }
}
