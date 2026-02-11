use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Setup, TableData, TableView, ViewData, ViewTest, view_test},
};

#[derive(Debug)]
struct Node {
    open:  bool,
    value: String,
    leafs: Vec<Node>,
}

#[view_test]
pub struct MenuView {
    #[init]
    table: TableView,
}

impl Setup for MenuView {
    fn setup(self: Weak<Self>) {
        self.table.set_data_source(self).place().back();
    }
}

impl TableData for MenuView {
    fn cell_height(self: Weak<Self>, _: usize) -> f32 {
        110.0
    }

    fn variable_height(self: Weak<Self>) -> bool {
        true
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        5
    }
}

impl ViewTest for MenuView {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        Ok(())
    }
}
