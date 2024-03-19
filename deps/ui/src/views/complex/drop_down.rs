use std::{any::Any, ops::Deref};

use gm::{flat::Size, Color, Toggle};
use itertools::Itertools;
use refs::{Own, Weak};
use ui_proc::view;
use vents::Event;

use crate::{
    view::{ViewData, ViewFrame, ViewSubviews, ViewTouch},
    Button, CollectionData, CollectionView, Label, Sub, ToLabel, View, ViewSetup,
};

mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct DropDown {
    #[link = tapped]
    button:  Sub<Button>,
    label:   Sub<Label>,
    table:   Sub<CollectionView>,
    values:  Vec<String>,
    opened:  bool,
    changed: Event<String>,
}

impl DropDown {
    pub fn on_changed(&self, action: impl FnMut(String) + 'static) {
        self.changed.val(action)
    }

    pub fn text(&self) -> &str {
        self.label.text()
    }

    pub fn set_values<Values, Val>(&mut self, values: Values)
    where
        Val: ToLabel,
        Values: IntoIterator<Item = Val>, {
        let values = values.into_iter().map(|a| a.to_label()).collect_vec();
        self.label.set_text(values.first().unwrap());
        self.values = values;
        let table_size = (self.width(), self.height() * self.number_of_cells() as f32);
        self.table.set_size(table_size);
    }

    fn tapped(&mut self) {
        if self.opened.toggle() {
            self.label.set_hidden(false);
            self.button.set_hidden(false);
            self.table.set_hidden(true);
        } else {
            self.label.set_hidden(true);
            self.button.set_hidden(true);
            self.table.set_hidden(false);
            let table_height = self.height() * self.number_of_cells() as f32;
            let table_size = (self.width(), table_height);
            self.table.set_size(table_size);
            self.table.reload_data();

            if self.superview().height() - self.max_y() < table_height {
                let y = -table_height + self.height();
                self.table.set_y(y);
            } else {
                self.table.set_y(0);
            }
        }
    }

    pub fn enable_editing(&mut self) -> &mut Self {
        self.button.enable_touch();
        self.set_color(Color::LIGHT_GRAY);
        self
    }

    pub fn disable_editing(&mut self) -> &mut Self {
        self.button.disable_touch();
        self.set_color(Color::CLEAR);
        self
    }
}

impl ViewSetup for DropDown {
    fn setup(mut self: Weak<Self>) {
        self.button.place().back();

        self.label.place().back();

        self.table.set_data_source(self.deref());
        self.table.set_hidden(true);
    }
}

impl CollectionData for DropDown {
    fn number_of_cells(&self) -> usize {
        self.values.len()
    }

    fn make_cell(&self) -> Own<dyn View> {
        let mut label = Label::new();
        label.label += "DropDown cell: ";
        label
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let label = cell.downcast_mut::<Label>().unwrap();
        label.set_text(&self.values[index]);
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (self.height(), self.height()).into()
    }

    fn cell_selected(&mut self, index: usize) {
        let val = &self.values[index];
        self.label.set_text(val);
        self.changed.trigger(val.clone());
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
