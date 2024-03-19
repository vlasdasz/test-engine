use gm::LossyConvert;
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
        self.scroll.on_scroll.sub(move || {
            self.layout_cells();
        });

        self.size_changed().sub(move || {
            self.layout_cells();
        })
    }
}

impl TableView {
    pub fn set_data_source(mut self: Weak<Self>, data: &(impl TableData + 'static)) -> Weak<Self> {
        self.data = weak_from_ref(data);
        self
    }
}

impl TableView {
    fn layout_cells(mut self: Weak<Self>) {
        self.scroll.remove_all_subviews();

        if self.height() <= 0.0 {
            return;
        }

        let number_of_cells = self.data.number_of_cells();
        let cell_height = self.data.cell_height();

        let total_height = number_of_cells as f32 * cell_height;
        self.scroll.content_size.height = total_height;

        let number_of_cells_fits: usize = (self.height() / cell_height).ceil().lossy_convert();

        let offset = self.scroll.content_offset.y;

        let first_index: usize = (-offset / cell_height).floor().lossy_convert();

        let mut last_index = first_index + number_of_cells_fits + 1;

        if last_index > number_of_cells {
            last_index = number_of_cells;
        }

        for i in first_index..last_index {
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
