use gm::{Color, flat::Size};
use refs::Weak;
use ui_proc::view;
use vents::OnceEvent;

use crate::{ModalView, Setup, has_data::HasText, view::ViewData};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::{Anchor::Width, Button, Label, ViewSubviews};

#[view]
pub struct Consent {
    event: OnceEvent<bool>,

    #[init]
    label:         Label,
    ok_button:     Button,
    cancel_button: Button,
}

impl ModalView<String, bool> for Consent {
    fn modal_event(&self) -> &OnceEvent<bool> {
        &self.event
    }

    fn modal_size() -> Size {
        (280, 200).into()
    }

    fn setup_input(mut self: Weak<Self>, input: String) {
        self.label.set_text(input);
    }
}

impl Consent {
    pub fn ask(message: impl Into<String>, callback: impl FnOnce(bool) + 'static + Send) {
        Self::show_modally_with_input(message.into(), callback);
    }

    pub async fn ask_async(message: impl ToString) -> bool {
        Self::show_modally_async(message.to_string()).await
    }
}

impl Setup for Consent {
    fn setup(mut self: Weak<Self>) {
        self.set_corner_radius(10).set_border_color(Color::BLACK);

        self.label.place().lrt(10).h(140);
        self.label.set_text_size(28);
        self.label.multiline = true;

        self.ok_button.place().h(50).br(2).relative(Width, self, 0.5);
        self.ok_button
            .set_text("OK")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::BLUE);

        self.ok_button.on_tap(move || self.hide_modal(true));

        self.cancel_button.place().h(50).bl(2).relative(Width, self, 0.5);
        self.cancel_button
            .set_text("Cancel")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::RED);

        self.cancel_button.on_tap(move || self.hide_modal(false));

        self.outline(Color::BLACK);
    }
}
