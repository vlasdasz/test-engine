use gm::Color;
use refs::{ToWeak, Weak};
use ui::{
    input::{ControlButton, KeyboardButton, UIEvents},
    view, SubView, UIManager, ViewCallbacks, ViewData, ViewSetup, ViewTouch,
};

use crate::Label;

#[view]
#[derive(Default)]
pub struct TextField {
    label: SubView<Label>,
}

impl TextField {
    pub fn text(&self) -> &str {
        self.label.text()
    }

    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        self.label.set_text(text);
        self
    }
}

impl ViewSetup for TextField {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.on_touch.sub(move |touch| {
            if touch.is_began() {
                self.set_selected(true);
            }
        });

        self.set_color(Color::LIGHT_GRAY);
        self.label.place.as_background();
    }
}

impl ViewCallbacks for TextField {
    fn on_selection_changed(&mut self, selected: bool) {
        if selected {
            UIManager::get().open_keyboard = true;
            let mut this = self.weak();
            UIEvents::get().key_pressed.sub(move |key| {
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
            UIManager::get().close_keyboard = true;
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
