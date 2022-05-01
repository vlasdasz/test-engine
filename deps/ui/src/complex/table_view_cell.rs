use rtools::{Boxed, Rglica, ToRglica};

use crate::{
    impl_view, view,
    view::{ViewFrame, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks,
};

pub trait TableViewData {
    fn make_cell(self) -> Box<dyn TableViewCell<Self>>;
}

pub trait TableViewCell<T>: View {
    fn set_data(&mut self, data: T);
}

#[view]
#[derive(Debug, Default)]
pub struct StringCell {
    data:  String,
    label: Rglica<Label>,
}

impl_view!(StringCell);

impl ViewCallbacks for StringCell {
    fn setup(&mut self) {
        self.label = self.add_view();
        self.label.set_text(self.data.clone());
    }

    fn layout(&mut self) {
        self.label.place().as_background()
    }
}

impl TableViewCell<String> for StringCell {
    fn set_data(&mut self, data: String) {
        self.label.set_text(data);
    }
}

impl TableViewData for String {
    fn make_cell(self) -> Box<dyn TableViewCell<Self>> {
        let mut cell = StringCell::boxed();
        cell.data = self;
        cell
    }
}
