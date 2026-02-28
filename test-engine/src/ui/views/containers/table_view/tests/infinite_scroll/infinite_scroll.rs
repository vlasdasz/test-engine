use std::any::Any;

use anyhow::Result;
use refs::{Own, Weak};
use ui::{Setup, TableData, View, ViewData, ViewTest, cast_cell, view_test};

use crate::{
    self as test_engine,
    ui::{TableView, views::containers::table_view::tests::infinite_scroll::infinite_cell::InfiniteCell},
};

#[view_test]
struct InfiniteScrollTest {
    data: Vec<usize>,

    #[init]
    table: TableView,
}

impl Setup for InfiniteScrollTest {
    fn setup(mut self: Weak<Self>) {
        self.table.set_data_source(self).place().back();
        self.data = (1..20).into_iter().collect();
    }
}

impl TableData for InfiniteScrollTest {
    fn cell_height(self: Weak<Self>, _index: usize) -> f32 {
        200.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        self.data.len()
    }

    fn make_cell(self: Weak<Self>, _index: usize) -> Own<dyn View> {
        InfiniteCell::new()
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        cast_cell!(InfiniteCell).set_text(self.data[index]);
    }
}

impl ViewTest for InfiniteScrollTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        crate::ui_test::record_ui_test();
        Ok(())
    }
}
