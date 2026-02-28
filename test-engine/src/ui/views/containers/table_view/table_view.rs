use refs::Weak;
use ui::{
    __ViewInternalTableData, ScrollView, Setup, ViewCallbacks, ViewData, ViewFrame,
    ViewSubviews, ViewTouch, WeakView, view,
};

use crate::{
    self as test_engine,
    ui::views::containers::table_view::layout::{layout_same_sized_cells, layout_variable_sized_cells},
};

#[view]
pub struct TableView {
    pub(super) data:    Weak<dyn __ViewInternalTableData>,
    #[educe(Default = 1)]
    pub(super) columns: usize,

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
    fn layout_cells(self: Weak<Self>) {
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

        if self.data.__variable_height() {
            layout_variable_sized_cells(self, number_of_cells);
        } else {
            layout_same_sized_cells(self, number_of_cells);
        }
    }

    pub(super) fn add_cell(mut self: Weak<Self>, index: usize) -> WeakView {
        let mut cell = self.data.__make_cell(index);

        let label = format!("TableView cell: {}", cell.label());
        cell.set_label(label);

        let mut cell = self.scroll.add_subview(cell);

        self.data.__setup_cell(cell.as_any_mut(), index);

        cell.enable_touch_low_priority();
        cell.touch().up_inside.sub(move || {
            self.data.__cell_selected(index);
        });

        cell
    }
}
