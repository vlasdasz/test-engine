use std::{fmt::Debug, marker::PhantomData};

use crate::{
    complex::table_view_cell::TableViewData,
    view::{ViewFrame, ViewTemplates},
    View, ViewBase,
};

#[derive(Default, Debug)]
pub struct TableView<T: TableViewData> {
    base:          ViewBase,
    _phantom_data: PhantomData<T>,
}

impl<T: Debug + Default + TableViewData> TableView<T> {
    pub fn set_data(&mut self, data: Vec<T>) {
        self.remove_all_subviews();
        for data in data {
            self.add_boxed(data.make_cell());
        }
    }
}

impl<T: Debug + Default + TableViewData> View for TableView<T> {
    fn layout(&mut self) {
        self.place().all_vertically()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
