use refs::Weak;
use ui::{view, SubView, ViewSetup};

use crate::Label;

#[view]
#[derive(Default)]
pub struct StringCell {
    data:  String,
    label: SubView<Label>,
}

impl ViewSetup for StringCell {
    fn setup(mut self: Weak<Self>) {
        let data = self.data.clone();
        self.label.set_text(data);
        self.label.place.as_background();
    }
}

impl StringCell {
    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }
}
