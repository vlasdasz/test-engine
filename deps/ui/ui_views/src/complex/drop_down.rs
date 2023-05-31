use gm::flat::Size;
use refs::{Own, ToOwn, ToWeak, Weak};
use ui::{view, SubView, View, ViewSetup};

use crate::{collection_data, CollectionData, CollectionView, Label};

#[view]
pub struct DropDown {
    label:  SubView<Label>,
    table:  SubView<CollectionView>,
    values: Vec<String>,
}

impl DropDown {
    pub fn set_values(mut self: Weak<Self>, values: Vec<String>) {
        self.label.set_text(values.first().unwrap());
        self.values = values;
        self.table.reload_data();
    }
}

impl ViewSetup for DropDown {
    fn setup(mut self: Weak<Self>) {
        self.table.data_source = collection_data!(self);
        self.table.reload_data();
    }
}

impl CollectionData for DropDown {
    fn number_of_cells(&self) -> usize {
        self.values.len()
    }

    fn cell_for_index(&self, index: usize) -> Own<dyn View> {
        Label::from(&self.values[index]).to_own()
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (28, 28).into()
    }
}

impl From<Vec<String>> for DropDown {
    fn from(value: Vec<String>) -> Self {
        let mut new = Self::default();
        new.values = value;
        new
    }
}
