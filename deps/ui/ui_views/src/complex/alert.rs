use gm::{flat::Size, Color};
use refs::Weak;
use ui::{view, ModalView, OnceEvent, SubView, ViewData, ViewSetup};

use crate::{Button, MultilineLabel};

#[view]
pub struct Alert {
    label:     SubView<MultilineLabel>,
    ok_button: SubView<Button>,
    event:     OnceEvent,
}

impl Alert {
    pub fn show(message: impl ToString) {
        Self::show_modally(message.to_string(), |()| {});
    }
}

impl ViewSetup for Alert {
    fn setup(mut self: Weak<Self>) {
        self.set_corner_radius(10).set_border_color(Color::BLACK);

        self.label.place.lrt(10).h(140);
        self.label.set_text_size(28);

        self.ok_button.place.h(28).lrb(-1);
        self.ok_button
            .set_text("OK")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::BLUE);

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
