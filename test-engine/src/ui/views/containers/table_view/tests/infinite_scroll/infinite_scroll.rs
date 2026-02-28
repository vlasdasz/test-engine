use std::any::Any;

use anyhow::Result;
use gm::color::BLACK;
use refs::{Own, Weak};
use ui::{Setup, TableData, View, ViewData, ViewTest, cast_cell, view_test};

use crate::{
    self as test_engine,
    ui::{
        TableView,
        views::containers::table_view::tests::infinite_scroll::{
            basic_scroll::test_basic_scroll, infinite_cell::InfiniteCell,
        },
    },
};

#[view_test]
pub(super) struct InfiniteScrollTest {
    pub(super) test_string: String,

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
            .t(120)
            .b(120)
            .lr(0);
        self.data = (0..=199).collect();
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

    fn cell_selected(mut self: Weak<Self>, index: usize) {
        self.test_string.push_str(&format!("|{index}|"));
    }
}

impl ViewTest for InfiniteScrollTest {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        test_basic_scroll(view)?;

        crate::ui_test::record_ui_test();

        Ok(())
    }
}
