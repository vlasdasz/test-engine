use refs::{weak_from_ref, Weak};
use ui::{view, Sub, ViewCallbacks, ViewData, ViewFrame, ViewSetup, ViewSubviews};

use crate::TableData;

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use crate::ScrollView;

#[view]
pub struct TableView {
    #[derivative(Debug = "ignore")]
    data:       Weak<dyn TableData>,
    pub scroll: Sub<ScrollView>,
}

impl ViewCallbacks for TableView {
    fn update(&mut self) {
        self.scroll.content_size.width = self.width();
    }
}

impl ViewSetup for TableView {
    fn setup(self: Weak<Self>) {
        self.scroll.place().back();
    }
}

impl TableView {
    pub fn set_data_source(mut self: Weak<Self>, data: &(impl TableData + 'static)) -> Weak<Self> {
        self.data = weak_from_ref(data);
        self
    }
}

impl TableView {
    pub fn layout_cells(mut self: Weak<Self>) {
        let number_of_cells = self.data.number_of_cells() as f32;

        let total_height = number_of_cells * self.data.cell_height();
        self.scroll.content_size.height = total_height;

        for i in 0..self.data.number_of_cells() {
            let mut cell = self.data.make_cell();

            cell.place()
                .h(self.data.cell_height())
                .t(i as f32 * self.data.cell_height())
                .lr(0);

            let label = format!("TableView cell: {}", cell.label());
            cell.set_label(label);

            let mut cell = self.scroll.add_subview(cell);

            self.data.setup_cell(cell.as_any_mut(), i);
        }
    }
}
