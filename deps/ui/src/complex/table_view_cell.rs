use rtools::{Rglica, ToRglica};

use crate::{
    impl_view, view,
    view::{ViewFrame, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default, Debug)]
pub struct StringCell {
    data:  String,
    label: Rglica<Label>,
}

impl_view!(StringCell);

impl ViewCallbacks for StringCell {
    fn setup(&mut self) {
        self.label = self.add_view();
        self.label.set_text(self.data.clone());
        self.label.make_layout(|l| l.as_background());
    }
}

impl StringCell {
    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }
}
