use std::{any::Any, ops::Deref};

use gm::{flat::Size, Color, LossyConvert, Toggle};
use refs::{weak_from_ref, Own, Weak};
use ui_proc::view;
use vents::Event;

use crate::{
    view::{ViewData, ViewFrame, ViewSubviews, ViewTouch},
    Button, CollectionData, CollectionView, InputView, Label, ToLabel, View, ViewSetup,
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct DropDown {
    values:  Vec<String>,
    opened:  bool,
    changed: Event<String>,

    #[init]
    button: Button,
    label:  Label,
    table:  CollectionView,
}

impl DropDown {
    pub fn on_changed(&self, action: impl FnMut(String) + 'static) {
        self.changed.val(action);
    }

    pub fn set_values<Values, Val>(&mut self, values: Values)
    where
        Val: ToLabel,
        Values: IntoIterator<Item = Val>, {
        let values = values.into_iter().map(|a| a.to_label()).collect();
        self.values = values;
        self.label.set_text(self.values.first().unwrap());
        let table_size = (
            self.width(),
            self.height() * self.number_of_cells().lossy_convert(),
        );
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
            let table_height = self.height() * self.number_of_cells().lossy_convert();
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
}

impl InputView for DropDown {
    fn set_title(&mut self, _title: &str) {
        unimplemented!("DropDown doesn't have title")
    }

    fn text(&self) -> &str {
        self.label.text()
    }

    fn enable_editing(&mut self) {
        self.button.enable_touch();
        self.set_color(Color::LIGHT_GRAY);
    }

    fn disable_editing(&mut self) {
        self.button.disable_touch();
        self.set_color(Color::CLEAR);
    }

    fn as_input_view(&self) -> Weak<dyn InputView> {
        weak_from_ref(self as &dyn InputView)
    }
}

impl ViewSetup for DropDown {
    fn setup(mut self: Weak<Self>) {
        self.button.place().back();
        self.button.on_tap(move || self.tapped());

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
        label.view_label += "DropDown cell: ";
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
