use dispatch::on_main;
use gm::Color;
use refs::{Own, ToWeak, Weak};
use tokio::sync::oneshot::channel;
use ui::{view, Event, SubView, UIManager, ViewData, ViewSetup, ViewSubviews};

use crate::{Button, MultilineLabel};

#[view]
pub struct Alert {
    label:         SubView<MultilineLabel>,
    ok_button:     SubView<Button>,
    cancel_button: SubView<Button>,
    message:       String,
    agreed:        Event<bool>,
}

impl Alert {
    fn prepare(message: impl ToString) -> Weak<Self> {
        let mut alert = Own::<Self>::default();
        alert.message = message.to_string();
        let res = alert.weak();
        UIManager::root_view().add_subview(alert);
        res
    }

    pub async fn ask(message: impl ToString) -> bool {
        let message = message.to_string();

        let (send, recv) = channel::<bool>();

        on_main(move || {
            let alert = Self::prepare(message);

            alert.agreed.once(move |agreed| {
                send.send(agreed).expect("BUG: Failed to send alert status.");
            });
        });

        recv.await.expect("BUG: Failed to receive alert status.")
    }

    pub fn show(message: impl ToString) {
        let message = message.to_string();
        on_main(|| {
            Self::prepare(message);
        })
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
        self.place.size(280, 200).center();
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

        self.ok_button.on_tap.sub(move || {
            self.remove_from_superview();
            self.agreed.trigger(true);
        });

        self.cancel_button.place.size(101, 20).br(-1);
        self.cancel_button
            .set_text("Cancel")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::RED);

        self.cancel_button.on_tap.sub(move || {
            self.remove_from_superview();
            self.agreed.trigger(false);
        });

        self.set_message();
    }
}
