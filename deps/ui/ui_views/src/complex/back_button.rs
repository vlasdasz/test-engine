use refs::Weak;
use ui::{view, SubView, ViewController, ViewSetup};

use crate::Button;

#[view]
pub struct BackButton {
    button: SubView<Button>,
}

impl ViewSetup for BackButton {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Back");
        self.button.place.back();

        self.button.on_tap.sub(move || {
            self.navigation().pop();
        });
    }
}
