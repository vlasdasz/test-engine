use std::any::Any;

use anyhow::Result;
use refs::{Own, Weak};
use ui::{Label, Setup, TableData, View, ViewData, ViewTest, cast_cell, view_test};

use crate::{self as test_engine, ui::TableView};

#[view_test]
struct InfiniteScrollTest {
    data: Vec<usize>,

    #[init]
    table: TableView,
}

impl Setup for InfiniteScrollTest {
    fn setup(mut self: Weak<Self>) {
        self.table.set_data_source(self).place().back();
        self.data = vec![1, 2, 3, 4, 5];
    }
}

impl TableData for InfiniteScrollTest {
    fn number_of_cells(self: Weak<Self>) -> usize {
        self.data.len()
    }

    fn make_cell(self: Weak<Self>, _index: usize) -> Own<dyn View> {
        Label::new()
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        cast_cell!(Label).set_text(self.data[index]);
    }
}

impl ViewTest for InfiniteScrollTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        // record_ui_test();
        Ok(())
    }
}
