use rtools::{Boxed, Rglica};

use crate::{
    view::{ViewFrame, ViewTemplates},
    Label, View, ViewBase,
};

pub trait TableViewData {
    fn make_cell(self) -> Box<dyn TableViewCell<Self>>;
}

pub trait TableViewCell<T>: View {
    fn set_data(&mut self, data: T);
}

#[derive(Debug, Default)]
pub struct StringCell {
    base:  ViewBase,
    data:  String,
    label: Rglica<Label>,
}

impl View for StringCell {
    fn setup(&mut self) {
        self.label = self.add_view();
        self.label.set_text(self.data.clone());
    }

    fn layout(&mut self) {
        self.label.place().as_background()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl TableViewCell<String> for StringCell {
    fn set_data(&mut self, data: String) {
        self.label.set_text(data)
    }
}

impl TableViewData for String {
    fn make_cell(self) -> Box<dyn TableViewCell<Self>> {
        let mut cell = StringCell::boxed();
        cell.data = self;
        cell
    }
}
