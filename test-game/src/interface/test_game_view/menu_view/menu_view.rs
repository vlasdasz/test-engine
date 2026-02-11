use std::any::Any;

use anyhow::Result;
use test_engine::{
    gm::Toggle,
    refs::{Own, Weak},
    ui::{Setup, TableData, TableView, View, ViewData, ViewFrame, ViewTest, cast_cell, view_test},
};

use crate::interface::test_game_view::{Node, NodeCell};

#[view_test]
pub struct MenuView {
    root: Node,

    #[init]
    table: TableView,
}

impl Setup for MenuView {
    fn setup(mut self: Weak<Self>) {
        // UIManager::override_scale(2.0);

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

    fn inspect(self: Weak<Self>) {
        dbg!(&self.frame());
    }
}

impl TableData for MenuView {
    fn cell_height(self: Weak<Self>, _: usize) -> f32 {
        50.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        self.root.length()
    }

    fn make_cell(self: Weak<Self>, _: usize) -> Own<dyn View> {
        NodeCell::new()
    }

    fn setup_cell(mut self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        let cell = cast_cell!(NodeCell);
        cell.set_node(self.root.val_at_index(index));
    }

    fn cell_selected(mut self: Weak<Self>, index: usize) {
        self.root.val_at_index(index).open.toggle();
        self.root.update_indices(0, 0);
        dbg!(&self.root);
        self.table.reload_data();
    }
}

impl ViewTest for MenuView {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        // record_ui_test();

        Ok(())
    }
}
