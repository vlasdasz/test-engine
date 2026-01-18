use std::any::Any;

use test_engine::{
    refs::{Own, Weak},
    ui::{LayoutRule, Placer, Setup, TableData, TableView, View, ViewData, view},
};

#[view]
pub struct PlacerView {
    view_id: String,

    rules: Vec<LayoutRule>,

    #[init]
    table: TableView,
}

impl Setup for PlacerView {
    fn setup(self: Weak<Self>) {
        self.place().all_ver();
        self.table.set_data_source(self);
    }
}

impl PlacerView {
    pub fn set_placer(mut self: Weak<Self>, id: &str, placer: &Placer) {
        self.view_id = id.to_string();
        self.rules = placer.get_rules().clone();
    }
}

impl TableData for PlacerView {
    fn cell_height(self: Weak<Self>) -> f32 {
        40.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        self.rules.len()
    }

    fn make_cell(self: Weak<Self>, index: usize) -> Own<dyn View> {
        todo!()
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        todo!()
    }
}
