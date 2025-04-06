use std::{any::Any, ops::Deref};

use gm::{LossyConvert, Toggle, flat::Size};
use refs::{Own, Weak};
use ui_proc::view;
use vents::Event;

use crate::{
    Button, CollectionData, CollectionView, Label, Setup, ToLabel, View,
    has_data::HasText,
    view::{ViewData, ViewFrame, ViewSubviews},
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct DropDown<T: 'static> {
    values:  Vec<T>,
    opened:  bool,
    changed: Event<T>,

    custom_format: Option<Box<dyn Fn(T) -> String>>,

    selected_index: usize,

    #[init]
    button: Button,
    label:  Label,
    table:  CollectionView,
}

impl<T: ToLabel + Clone + 'static> DropDown<T> {
    pub fn on_changed(&self, action: impl FnMut(T) + Send + 'static) {
        self.changed.val(action);
    }

    pub fn value(&self) -> &T {
        assert!(!self.values.is_empty());
        self.values.get(self.selected_index).unwrap()
    }

    pub fn set_values(&mut self, values: Vec<T>) {
        self.selected_index = 0;
        self.values = values;

        if self.values.is_empty() {
            self.label.set_text("");
            return;
        }

        let first = self.values.first().unwrap().clone();

        if let Some(format) = &self.custom_format {
            self.label.set_text(format(first));
        } else {
            self.label.set_text(first);
        }

        let size: Size = (
            self.width(),
            self.height() * self.number_of_cells().lossy_convert(),
        )
            .into();

        self.table.set_size(size.width, size.height);
    }

    pub fn custom_format(&mut self, format: impl Fn(T) -> String + 'static) {
        self.custom_format = Some(Box::new(format));
        self.set_values(self.values.clone());
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
            let table_height = self.height() * self.number_of_cells().lossy_convert();
            let width = self.width();
            self.table.set_size(width, table_height);
            self.table.reload_data();

            if self.superview().height() - self.max_y() < table_height {
                let y = -table_height + self.height();
                self.table.set_y(y);
            } else {
                self.table.set_y(0);
            }
        }
    }
}

impl<T: ToLabel + Clone + 'static> Setup for DropDown<T> {
    fn setup(mut self: Weak<Self>) {
        self.button.place().back();
        self.button.on_tap(move || self.tapped());

        self.label.place().back();

        self.table.set_data_source(self.deref());
        self.table.set_hidden(true);
    }
}

impl<T: ToLabel + Clone + 'static> CollectionData for DropDown<T> {
    fn number_of_cells(&self) -> usize {
        self.values.len()
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let label = cell.downcast_mut::<Label>().unwrap();

        let val = self.values[index].clone();

        if let Some(format) = &self.custom_format {
            label.set_text(format(val));
        } else {
            label.set_text(val);
        }
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (self.height(), self.height()).into()
    }

    fn make_cell(&self) -> Own<dyn View> {
        let mut label = Label::new();
        label.base_view_mut().view_label += "DropDown cell: ";
        label
    }

    fn cell_selected(&mut self, index: usize) {
        self.selected_index = index;
        let val = &self.values[index];
        if let Some(format) = &self.custom_format {
            self.label.set_text(format(val.clone()));
        } else {
            self.label.set_text(val.clone());
        }
        self.changed.trigger(val.clone());
        self.tapped();
    }
}
