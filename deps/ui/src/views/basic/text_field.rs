use gm::{
    ToF32,
    color::{BLACK, Color, GRAY, LIGHT_GRAY, LIGHTER_GRAY},
};
use refs::{Weak, weak_from_ref};
use ui_proc::view;
use vents::Event;
use window::NamedKey;

use crate::{
    Label, Setup, TextAlignment, TextFieldConstraint, ToLabel, UIEvents, UIManager, ViewCallbacks,
    has_data::HasText,
    text_field_constraint::AcceptChar,
    view::{ViewData, ViewFrame, ViewTouch},
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct TextField {
    pub(crate) constraint: Option<TextFieldConstraint>,

    placeholder:  String,
    text_color:   Color,
    placeholding: bool,
    is_editing:   bool,

    pub changed: Event<String>,

    pub editing_ended: Event<String>,

    #[init]
    label: Label,
}

impl Setup for TextField {
    fn setup(mut self: Weak<Self>) {
        self.text_color = BLACK;
        self.placeholding = true;
        self.label.place().back();
        self.label.set_text_color(LIGHTER_GRAY);
        self.set_color(LIGHT_GRAY);

        self.enable_touch();
    }
}

impl TextField {
    pub fn set_alignment(&mut self, alignment: TextAlignment) -> &mut Self {
        self.label.set_alignment(alignment);
        self
    }

    pub fn text(&self) -> &str {
        self.label.text()
    }

    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        let text = self.filter_constraint(text);

        if text.is_empty() && !self.placeholder.is_empty() {
            self.placeholding = true;
            self.label.set_text(self.placeholder.clone());
            self.label.set_text_color(LIGHTER_GRAY);
        } else {
            self.placeholding = false;
            self.label.set_text(&text);
            self.label.set_text_color(self.text_color);
        }

        self.changed.trigger(text);
        self
    }

    pub fn is_editing(&self) -> bool {
        self.is_editing
    }

    pub fn clear(&mut self) -> &mut Self {
        self.set_text("")
    }

    pub fn is_empty(&self) -> bool {
        self.label.text().is_empty()
    }

    fn filter_constraint(&mut self, text: impl ToLabel) -> String {
        match &self.constraint {
            Some(constraint) => constraint.filter(text),
            None => text.to_label(),
        }
    }

    pub fn float_only(&mut self) -> &mut Self {
        self.constraint = TextFieldConstraint::Float.into();
        self
    }

    pub fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self {
        let color = color.into();
        self.text_color = color;
        self.label.set_text_color(color);
        self
    }

    pub fn set_text_size(&mut self, size: impl ToF32) -> &mut Self {
        self.label.set_text_size(size);
        self
    }

    pub fn set_placeholder(&mut self, placeholder: impl ToLabel) -> &mut Self {
        self.placeholder = placeholder.to_label();
        if self.placeholding {
            self.label.set_text(self.placeholder.clone());
            self.label.set_text_color(GRAY);
        }
        self
    }
}

impl ViewCallbacks for TextField {
    fn on_selection_changed(&mut self, selected: bool) {
        let mut this = weak_from_ref(self);

        self.is_editing = selected;

        if selected {
            UIEvents::keyboard_key().val(this, |key| {
                if matches!(key, NamedKey::Enter) {
                    UIManager::unselect_view();
                }
            });

            UIEvents::keyboard_input().val(this, move |key| {
                if this.is_null() {
                    return;
                }

                let mut text = this.label.text().to_string();

                let backspace = key as u32 == 8;

                if this.is_selected() {
                    if backspace {
                        if this.placeholding {
                            return;
                        }
                        text.pop();
                    } else if this.constraint.accept_char(key, &text) {
                        if this.placeholding {
                            text = String::default();
                            this.placeholding = false;
                        }
                        text.push(key);
                    }
                }

                this.set_text(text);
                this.changed.trigger(this.label.text().to_string());
            });
            UIManager::open_keyboard(self.absolute_frame());
        } else {
            if let Some(string) = UIManager::close_keyboard() {
                self.set_text(string);
            }
            UIEvents::keyboard_input().unsibscribe(this);
            UIEvents::keyboard_key().unsibscribe(this);

            self.editing_ended.trigger(self.label.text().to_string());
        }

        self.label.set_color(if selected { GRAY } else { LIGHT_GRAY });
    }
}
