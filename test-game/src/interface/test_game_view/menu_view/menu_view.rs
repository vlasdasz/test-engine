use std::any::Any;

use anyhow::Result;
use test_engine::{
    gm::Toggle, refs::Weak, ui::{Label, Setup, TableData, TableView, ViewData, ViewTest, cast_cell, view_test}, ui_test::record_ui_test
};

use crate::interface::test_game_view::Node;

#[view_test]
pub struct MenuView {
    root: Node,

    #[init]
    table: TableView,
}

impl Setup for MenuView {
    fn setup(mut self: Weak<Self>) {
        self.table.set_data_source(self).place().back();

        self.root = Node::new(
            "Root",
            vec![
                Node::empty("A"),
                Node::new("Ooo", vec![Node::empty("a"), Node::empty("b")]),
                Node::empty("C"),
            ],
        )
    }
}

impl TableData for MenuView {
    fn cell_height(self: Weak<Self>, _: usize) -> f32 {
        50.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        self.root.length()
    }

    fn setup_cell(mut self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        let cell = cast_cell!(Label);
        let val = self.root.val_at_index(index).value.clone();
        cell.set_text(val);
    }

    fn cell_selected(mut self: Weak<Self>, index: usize) {
        self.root.val_at_index(index).open.toggle();
        self.root.update_indices(0);
        self.table.reload_data();
    }
}

impl ViewTest for MenuView {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        record_ui_test();

        Ok(())
    }
}
