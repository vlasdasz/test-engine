use std::any::Any;

use anyhow::Result;
use gm::color::BLACK;
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
        self.table.columns = 2;
        self.table
            .set_data_source(self)
            .set_border_color(BLACK)
            .set_border_width(5)
            .place()
            .t(200)
            .b(200)
            .lr(0);
        self.data = (1..=100).collect();
    }
}

impl TableData for InfiniteScrollTest {
    fn cell_height(self: Weak<Self>, _index: usize) -> f32 {
        80.0
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
