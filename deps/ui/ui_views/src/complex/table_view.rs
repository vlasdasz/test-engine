use refs::{Own, ToWeak, Weak};
use ui::{view, View, ViewSetup, ViewSubviews, ViewTouch};

#[view]
#[derive(Default)]
pub struct TableView {
    pub data_source: Weak<dyn TableViewDataSource>,
}

impl TableView {
    pub fn reload_data(&mut self) {
        self.remove_all_subviews();
        for i in 0..self.data_source.number_of_cells() {
            let cell = self.data_source.cell_for_index(i);
            cell.enable_touch();
            let mut this = self.weak();
            cell.on_touch_began.sub(move |_| this.data_source.cell_selected(i));
            self.add_subview(cell);
        }
    }
}

impl ViewSetup for TableView {
    fn setup(mut self: Weak<Self>) {
        self.place.all_ver();
    }
}

pub trait TableViewDataSource {
    fn number_of_cells(&self) -> usize;
    fn cell_for_index(&self, index: usize) -> Own<dyn View>;
    fn cell_selected(&mut self, index: usize);
}

#[macro_export]
macro_rules! data_source {
    ($source:ident) => {{
        use std::ops::DerefMut;

        use refs::ToWeak;
        ($source.deref_mut() as &mut dyn TableViewDataSource).weak()
    }};
}
