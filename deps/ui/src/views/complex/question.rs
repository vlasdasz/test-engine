use gm::{flat::Size, Color};
use refs::Weak;
use ui_proc::view;
use vents::OnceEvent;

use crate::{view::ViewData, ModalView, Sub, ViewSetup};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::{Button, Label};

#[view]
pub struct Question {
    label:         Sub<Label>,
    ok_button:     Sub<Button>,
    cancel_button: Sub<Button>,
    event:         OnceEvent<bool>,
}

impl ModalView<String, bool> for Question {
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

impl Question {
    pub fn ask(message: impl ToString, callback: impl FnOnce(bool) + 'static + Send) {
        Self::show_modally(message.to_string(), callback);
    }

    pub async fn ask_async(message: impl ToString) -> bool {
        Self::show_modally_async(message.to_string()).await
    }
}

impl ViewSetup for Question {
    fn setup(mut self: Weak<Self>) {
        self.set_corner_radius(10).set_border_color(Color::BLACK);

        self.label.place().lrt(10).h(140);
        self.label.set_text_size(28);

        self.ok_button.place().size(101, 20).br(-1);
        self.ok_button
            .set_text("OK")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::BLUE);

        self.ok_button.on_tap(move || self.hide_modal(true));

        self.cancel_button.place().size(101, 20).bl(-1);
        self.cancel_button
            .set_text("Cancel")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::RED);

        self.cancel_button.on_tap(move || self.hide_modal(false));
    }
}
