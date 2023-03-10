use refs::Weak;
use ui::{view, SubView, UIManager, ViewController, ViewSetup};

use crate::Button;

#[view]
pub struct BackButton {
    button: SubView<Button>,
}

impl ViewSetup for BackButton {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Back");
        self.button.place.background();

        self.button.on_tap.sub(|| {
            UIManager::touch_root().pop();
        });
    }
}
