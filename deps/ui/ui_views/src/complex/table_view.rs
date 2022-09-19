use rtools::Rglica;
use ui::{view, View, ViewCallbacks, ViewLayout, ViewSubviews};

#[view]
#[derive(Default)]
pub struct TableView {
    pub data_source: Rglica<dyn TableViewDataSource>,
}

impl TableView {
    pub fn reload_data(&mut self) {
        self.remove_all_subviews();
        for i in 0..self.data_source.number_of_cells() {
            self.add_subview(self.data_source.cell_for_index(i));
        }
    }
}

impl ViewCallbacks for TableView {
    fn setup(&mut self) {
        self.place().all_ver();
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
