use dispatch::from_main;
use gm::{flat::Size, Color};
use refs::Weak;
use tokio::sync::oneshot::{channel, Receiver, Sender};
use ui::{view, ModalView, SubView, ViewData, ViewSetup};

use crate::{Button, MultilineLabel};

#[view]
pub struct Alert {
    label:         SubView<MultilineLabel>,
    ok_button:     SubView<Button>,
    cancel_button: SubView<Button>,
    message:       String,
    sender:        Option<Sender<bool>>,
}

impl ModalView<String, bool> for Alert {
    fn modal_size() -> Size {
        (280, 200).into()
    }

    fn send(&mut self) -> Sender<bool> {
        self.sender.take().unwrap()
    }

    fn recv(&mut self) -> Receiver<bool> {
        let (s, r) = channel();
        self.sender = s.into();
        r
    }

    fn setup_input(mut self: Weak<Self>, input: String) {
        self.label.set_text(input);
    }
}

impl Alert {
    pub async fn ask(message: impl ToString + 'static + Send) -> bool {
        from_main(move || Self::show_modally(message.to_string())).await.await.unwrap()
    }
}

impl Alert {
    fn set_message(&mut self) {
        let message = self.message.clone();
        self.label.set_text(message);
    }
}

impl ViewSetup for Alert {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::WHITE)
            .set_corner_radius(10)
            .set_border_color(Color::BLACK);

        self.label.place.lrt(10).h(140);
        self.label.set_text_size(28);

        self.ok_button.place.size(101, 20).bl(-1);
        self.ok_button
            .set_text("OK")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::BLUE);

        self.ok_button.on_tap.sub(move || self.hide_modal(true));

        self.cancel_button.place.size(101, 20).br(-1);
        self.cancel_button
            .set_text("Cancel")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::RED);

        self.cancel_button.on_tap.sub(move || self.hide_modal(false));

        self.set_message();
    }
}
