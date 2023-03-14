use gm::Color;
use refs::{ToWeak, Weak};
use ui::{
    input::{ControlButton, KeyboardButton, UIEvents},
    view, SubView, UIManager, ViewCallbacks, ViewData, ViewSetup, ViewTouch,
};

use crate::{
    basic::{text_field_constraint::AcceptChar, TextFieldConstraint},
    Label,
};

#[view]
pub struct TextField {
    label:                 SubView<Label>,
    pub(crate) constraint: Option<TextFieldConstraint>,
}

impl TextField {
    pub fn text(&self) -> &str {
        self.label.text()
    }

    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        let text = self.filter_constraint(text);
        self.label.set_text(text);
        self
    }

    fn filter_constraint(&mut self, text: impl ToString) -> String {
        match &self.constraint {
            Some(constraint) => constraint.filter(text),
            None => text.to_string(),
        }
    }
}

impl ViewSetup for TextField {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.on_touch.val(move |touch| {
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
            UIEvents::get().key_pressed.val(move |key| {
                if this.is_selected() {
                    match key.button {
                        KeyboardButton::Letter(char) => {
                            if this.constraint.accept_char(char, this.label.text()) {
                                this.label.append_text(char);
                            }
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
            UIEvents::get().key_pressed.remove_subscribers();
        }

        self.set_color(if selected { Color::GRAY } else { Color::LIGHT_GRAY });
    }
}

impl Drop for TextField {
    fn drop(&mut self) {
        UIEvents::get().key_pressed.remove_subscribers()
    }
}
