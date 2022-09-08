use rtools::{Rglica, ToRglica};

use crate::{view, view::ViewSubviews, Label, View, ViewBase, ViewCallbacks, ViewLayout};

#[view]
#[derive(Default, Debug)]
pub struct StringCell {
    data:  String,
    label: Rglica<Label>,
}

impl ViewCallbacks for StringCell {
    fn setup(&mut self) {
        self.label = self.add_view();
        self.label.set_text(self.data.clone());
        self.label.place().as_background();
    }
}

impl StringCell {
    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }
}
