use gm::{flat::Size, Color};
use refs::{Own, ToOwn, ToWeak, Weak};
use rtools::{MapVec, Toggle};
use ui::{view, SubView, ToLabel, View, ViewData, ViewFrame, ViewSetup};

use crate::{collection_data, link_button, Button, CollectionData, CollectionView, Label};

#[view]
pub struct DropDown {
    button: SubView<Button>,
    label:  SubView<Label>,
    table:  SubView<CollectionView>,
    values: Vec<String>,
    opened: bool,
}

impl DropDown {
    pub fn set_values(&mut self, values: Vec<impl ToLabel>) {
        let values = values.map(|a| a.to_label());
        self.label.set_text(values.first().unwrap());
        self.values = values;
        self.table.reload_data();
        let table_size = (self.width(), self.height() * self.number_of_cells() as f32);
        self.table.set_size(table_size);
    }

    fn tapped(&mut self) {
        if self.opened.toggle() {
            self.label.set_hidden(false);
            self.table.set_hidden(true);
        } else {
            self.label.set_hidden(true);
            self.table.reload_data();
            self.table.set_hidden(false);
            let table_size = (self.width(), self.height() * self.number_of_cells() as f32);
            self.table.set_size(table_size);
        }
    }
}

impl ViewSetup for DropDown {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::WHITE);

        self.button.place.back();
        link_button!(self, button, tapped);

        self.label.place.back();

        self.table.data_source = collection_data!(self);
        self.table.set_color(Color::WHITE);
        self.table.set_hidden(true);
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
        (self.height(), self.height()).into()
    }

    fn cell_selected(&mut self, index: usize) {
        let val = &self.values[index];
        self.label.set_text(val);
        self.tapped();
    }
}

impl From<Vec<String>> for DropDown {
    fn from(value: Vec<String>) -> Self {
        DropDown {
            values: value,
            ..Default::default()
        }
    }
}
