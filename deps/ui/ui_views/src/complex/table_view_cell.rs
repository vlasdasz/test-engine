use ui::{view, SubView, ViewCallbacks};

use crate::Label;

#[view]
#[derive(Default)]
pub struct StringCell {
    data:  String,
    label: SubView<Label>,
}

impl ViewCallbacks for StringCell {
    fn setup(&mut self) {
        self.label.set_text(self.data.clone());
        self.label.place.as_background();
    }
}

impl StringCell {
    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }
}
