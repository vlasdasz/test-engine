use std::fmt::Debug;

use rtools::{Rglica, ToRglica};

use crate::{
    impl_view, view,
    view::{ViewFrame, ViewSubviews},
    View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default, Debug)]
pub struct TableView {
    pub data_source: Rglica<dyn TableViewDataSource>,
}
impl_view!(TableView);

impl TableView {
    pub fn reload_data(&mut self) {
        self.remove_all_subviews();
        for i in 0..self.data_source.number_of_cells() {
            self.add_boxed(self.data_source.cell_for_index(i));
        }
    }
}

impl ViewCallbacks for TableView {
    fn setup(&mut self) {
        self.make_tiling(|t| {
            t.ver();
        });
    }
}

pub trait TableViewDataSource {
    fn number_of_cells(&self) -> usize;
    fn cell_for_index(&self, index: usize) -> Box<dyn View>;
}

#[macro_export]
macro_rules! data_source {
    ($source:ident) => {
        ($source as &mut dyn TableViewDataSource).to_rglica()
    };
}
