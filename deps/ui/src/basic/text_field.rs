use rtools::{Rglica, ToRglica};

use crate::{
    input::UIEvents, view, view::ViewSubviews, Label, SubView, View, ViewBase, ViewCallbacks, ViewLayout,
};

#[view]
#[derive(Default)]
pub struct TextField {
    label: SubView<Label>,
}

impl TextField {
    pub fn text(&self) -> &str {
        self.label.text()
    }
}

impl ViewCallbacks for TextField {
    fn setup(&mut self) {
        self.label.place().as_background();

        UIEvents::get().key_pressed.set(self, |this, event| {
            this.label.append_text(event.0);
        });
    }
}

impl Drop for TextField {
    fn drop(&mut self) {
        UIEvents::get().key_pressed.unsubscribe()
    }
}
