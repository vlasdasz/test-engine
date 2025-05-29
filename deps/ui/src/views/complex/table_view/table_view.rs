use gm::LossyConvert;
use refs::Weak;
use ui_proc::view;

use crate::{
    __ViewInternalTableData, Setup, View, ViewCallbacks, ViewTouch,
    view::{ViewData, ViewFrame, ViewSubviews},
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}
use crate::ScrollView;

#[view]
pub struct TableView {
    data: Weak<dyn __ViewInternalTableData>,

    #[init]
    pub scroll: ScrollView,
}

impl ViewCallbacks for TableView {
    fn update(&mut self) {
        let width = self.width();
        self.scroll.set_content_width(width);
    }
}

impl Setup for TableView {
    fn setup(self: Weak<Self>) {
        self.scroll.place().back();
        self.scroll.on_scroll.sub(move || {
            self.layout_cells();
        });

        self.size_changed().sub(move || {
            self.layout_cells();
        });
    }
}

impl TableView {
    pub fn set_data_source<T: __ViewInternalTableData + 'static>(
        mut self: Weak<Self>,
        data: Weak<T>,
    ) -> Weak<Self> {
        self.data = data;
        self
    }

    pub fn reload_data(self: Weak<Self>) {
        self.layout_cells();
    }
}

impl TableView {
    fn layout_cells(mut self: Weak<Self>) {
        self.scroll.remove_all_subviews();

        if self.height() <= 0.0 {
            return;
        }

        assert!(
            self.data.is_ok(),
            "TableView data source is not set. Use set_data_source method."
        );

        let number_of_cells = self.data.__number_of_cells();

        if number_of_cells == 0 {
            return;
        }

        let cell_height = self.data.__cell_height();

        let total_height = number_of_cells.lossy_convert() * cell_height;

        self.scroll.set_content_height(total_height);

        let number_of_cells_fits: usize = (self.height() / cell_height).ceil().lossy_convert();

        let offset = self.scroll.base_view().content_offset;

        let first_index: usize = (-offset / cell_height).floor().lossy_convert();

        let mut last_index = first_index + number_of_cells_fits + 1;

        if last_index > number_of_cells {
            last_index = number_of_cells;
        }

        let h = self.data.__cell_height();

        for i in first_index..last_index {
            let mut cell = self.data.__make_cell();

            let label = format!("TableView cell: {}", cell.label());
            cell.set_label(label);

            let mut cell = self.scroll.add_subview(cell);

            self.data.__setup_cell(cell.as_any_mut(), i);

            cell.place().h(h).t(i.lossy_convert() * h).lr(0);

            cell.enable_touch_low_priority();
            cell.touch().up_inside.sub(move || {
                self.data.__cell_selected(i);
            });
        }
    }
}
