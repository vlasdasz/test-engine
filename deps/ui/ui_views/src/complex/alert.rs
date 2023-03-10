use gm::Color;
use refs::{Own, ToWeak, Weak};
use ui::{view, Event, SubView, UIManager, ViewData, ViewSetup, ViewSubviews};

use crate::{Button, MultilineLabel};

#[view]
pub struct Alert {
    label:     SubView<MultilineLabel>,
    ok_button: SubView<Button>,
    message:   String,
    on_ok:     Event,
}

impl Alert {
    fn prepare(message: impl ToString) -> Weak<Self> {
        let mut alert = Own::<Self>::default();
        alert.message = message.to_string();
        let res = alert.weak();
        UIManager::root_view().add_subview(alert);
        res
    }

    pub fn ok(message: impl ToString, mut on_ok: impl FnMut() + 'static) {
        let alert = Self::prepare(message);
        alert.on_ok.sub(move || {
            on_ok();
        });
    }

    pub fn show(message: impl ToString) {
        Self::prepare(message);
    }

    pub fn ask(_message: impl ToString, _on_ok: impl FnMut()) {}
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

        self.ok_button.place.size(202, 20).center_hor().b(-1);
        self.ok_button
            .set_text("OK")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::BLUE);

        self.ok_button.on_tap.sub(move || {
            self.remove_from_superview();
            self.on_ok.trigger(());
        });

        self.set_message();
    }
}
