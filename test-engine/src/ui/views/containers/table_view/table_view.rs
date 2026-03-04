use refs::{Weak, weak_from_ref};
use ui::{
    __ViewInternalTableData, ScrollView, Setup, UIEvent, ViewCallbacks, ViewData, ViewFrame, ViewSubviews,
    ViewTouch, WeakView, view,
};

use crate::{
    self as test_engine,
    ui::views::containers::table_view::layout::{
        layout_single_column_cells, layout_two_column_cells, layout_variable_sized_cells,
    },
};

#[view]
pub struct TableView {
    pub(super) data:    Weak<dyn __ViewInternalTableData>,
    #[educe(Default = 1)]
    pub(super) columns: usize,

    pub on_reload: UIEvent,

    #[init]
    pub(super) scroll: ScrollView,
}

impl ViewCallbacks for TableView {
    fn update(&mut self) {
        let width = self.width();
        self.scroll.set_content_width(width);
    }
}

impl Setup for TableView {
    fn setup(mut self: Weak<Self>) {
        self.scroll.place().back();
        self.scroll.on_scroll.sub(move || {
            self.layout_cells();
        });

        self.size_changed().sub(move || {
            dbg!("Size changed");
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

    pub fn set_columns(mut self: Weak<Self>, columns: usize) -> Weak<Self> {
        assert!(columns <= 2);

        self.columns = columns;
        self.scroll.set_content_offset(0);
        self.reload_data();

        self
    }

    pub fn reload_data(mut self: Weak<Self>) {
        self.layout_cells();
        self.on_reload.trigger(());
    }

    pub fn bottom_reached(&self) -> &UIEvent {
        &self.scroll.bottom_reached
    }
}

impl TableView {
    fn layout_cells(&mut self) {
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
            assert_eq!(
                self.columns, 1,
                "Variable height supported only for tables with 1 column"
            );
            layout_variable_sized_cells(self, number_of_cells);
        } else {
            match self.columns {
                1 => layout_single_column_cells(self, number_of_cells),
                2 => layout_two_column_cells(self, number_of_cells),
                _ => unimplemented!("More than TableView 2 columns is not supported yet"),
            }
        }
    }

    pub(super) fn add_cell(&self, index: usize) -> WeakView {
        let mut cell = self.data.__make_cell(index);

        let label = format!("TableView cell: {}", cell.label());
        cell.set_label(label);

        let mut cell = self.scroll.add_subview(cell);

        self.data.__setup_cell(cell.as_any_mut(), index);

        cell.enable_touch_low_priority();
        let mut weak = weak_from_ref(self);
        cell.touch().up_inside.sub(move || {
            weak.data.__cell_selected(index);
        });

        cell
    }
}
