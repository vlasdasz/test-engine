use std::ops::{Deref, DerefMut};

use rtools::{Rglica, ToRglica};

use crate::{
    impl_view, input::UIEvents, view, view::ViewSubviews, Label, View, ViewBase, ViewCallbacks, ViewLayout,
};

#[view]
#[derive(Default, Debug)]
pub struct TextField {
    label: Rglica<Label>,
}
impl_view!(TextField);

impl ViewCallbacks for TextField {
    fn setup(&mut self) {
        self.label = self.add_view();
        self.label.new_placer().as_background();

        UIEvents::get().on_key_pressed.set(self, |this, event| {
            this.label.append_text(event.0);
        });
    }
}

impl Drop for TextField {
    fn drop(&mut self) {
        UIEvents::get().on_key_pressed.unsubscribe()
    }
}

impl Deref for TextField {
    type Target = Label;
    fn deref(&self) -> &Label {
        self.label.deref()
    }
}

impl DerefMut for TextField {
    fn deref_mut(&mut self) -> &mut Label {
        self.label.deref_mut()
    }
}
