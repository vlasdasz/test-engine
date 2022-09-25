use rtools::Rglica;
use ui::{view, View, ViewCallbacks, ViewSubviews, ViewTouch};

#[view]
#[derive(Default)]
pub struct TableView {
    pub data_source: Rglica<dyn TableViewDataSource>,
}

impl TableView {
    pub fn reload_data(&mut self) {
        self.remove_all_subviews();
        for i in 0..self.data_source.number_of_cells() {
            let cell = self.data_source.cell_for_index(i);
            cell.enable_touch();
            cell.on_touch_began.set(self, move |this, _| this.data_source.cell_selected(i));
            self.add_subview(cell);
        }
    }
}

impl ViewCallbacks for TableView {
    fn setup(&mut self) {
        self.place.all_ver();
    }
}

pub trait TableViewDataSource {
    fn number_of_cells(&self) -> usize;
    fn cell_for_index(&self, index: usize) -> Box<dyn View>;
    fn cell_selected(&mut self, index: usize);
}

#[macro_export]
macro_rules! data_source {
    ($source:ident) => {{
        use rtools::ToRglica;
        ($source as &mut dyn TableViewDataSource).to_rglica()
    }};
}
