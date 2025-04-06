use gm::{
    color::{BLACK, BLUE, GRAY},
    flat::Size,
};
use refs::Weak;
use ui_proc::view;
use vents::OnceEvent;

use crate::{Button, Label, ModalView, Setup, has_data::HasText, view::ViewData};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct Alert {
    event: OnceEvent,

    #[init]
    label:     Label,
    ok_button: Button,
}

impl Alert {
    pub fn show(message: impl ToString) {
        Self::show_modally_with_input(message.to_string(), |()| {});
    }

    pub fn show_callback(message: impl ToString, callback: impl FnOnce() + Send + 'static) {
        Self::show_modally_with_input(message.to_string(), move |()| callback());
    }
}

impl Setup for Alert {
    fn setup(mut self: Weak<Self>) {
        self.set_corner_radius(10).set_border_color(BLACK);

        self.label.place().lrt(10).h(140);
        self.label.set_text_size(28);
        self.label.multiline = true;

        self.ok_button.place().h(28).lrb(-1);
        self.ok_button.set_text("OK").set_border_color(GRAY).set_text_color(BLUE);

        self.ok_button.on_tap(move || self.hide_modal(()));
    }
}

impl ModalView<String> for Alert {
    fn modal_event(&self) -> &OnceEvent<()> {
        &self.event
    }

    fn modal_size() -> Size {
        (280, 200).into()
    }

    fn setup_input(mut self: Weak<Self>, message: String) {
        self.label.set_text(message);
    }
}
