use refs::Weak;
use ui::{view, Sub, ViewController, ViewData, ViewSetup};
mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use crate::Button;

#[view]
pub struct BackButton {
    button: Sub<Button>,
}

impl ViewSetup for BackButton {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Back");
        self.button.place().back();

        self.button.on_tap(move || {
            self.navigation().pop();
        });
    }
}
