use refs::Weak;
use ui_proc::view;

use crate::{
    view::{ViewController, ViewData},
    Button, Sub, ViewSetup,
};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

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
