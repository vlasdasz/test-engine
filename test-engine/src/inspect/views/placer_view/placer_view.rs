use std::any::Any;

use refs::{Own, Weak};
use ui::{LayoutRule, Placer, Setup, TableData, TableView, UIEvent, View, ViewData};
use ui_proc::{cast_cell, view};

use crate::inspect::views::LayoutRuleCell;

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate::ui;
}

#[view]
pub struct PlacerView {
    pub rule_changed: UIEvent<(f32, usize)>,

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
        self.rules.clone_from(&placer.get_rules());
        self.table.reload_data();
    }
}

impl TableData for PlacerView {
    fn cell_height(self: Weak<Self>) -> f32 {
        50.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        self.rules.len()
    }

    fn make_cell(self: Weak<Self>, _index: usize) -> Own<dyn View> {
        LayoutRuleCell::new()
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        let cell = cast_cell!(LayoutRuleCell);
        cell.set_rule(self.rules[index].clone());
        cell.editing_ended.val(self, move |value| {
            self.rule_changed.trigger((value, index));
        });
    }
}
