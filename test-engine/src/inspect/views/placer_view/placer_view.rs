use std::any::Any;

use refs::{Own, Rglica, ToRglica, Weak};
use ui::{Placer, Setup, TableData, UIEvent, View, ViewData};
use ui_proc::{cast_cell, view};

use crate::{inspect::views::LayoutRuleCell, ui::TableView};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate::ui;
}

#[view]
pub struct PlacerView {
    pub rule_changed: UIEvent,

    placer: Rglica<Placer>,

    view_id: String,

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
        self.placer = placer.to_rglica();
        self.view_id = id.to_string();
        self.table.reload_data();
    }
}

impl TableData for PlacerView {
    fn cell_height(self: Weak<Self>, _: usize) -> f32 {
        50.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        if self.placer.is_null() {
            return 0;
        }
        self.placer.get_rules().len()
    }

    fn make_cell(self: Weak<Self>, _index: usize) -> Own<dyn View> {
        LayoutRuleCell::new()
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        if self.placer.is_null() {
            return;
        }
        let cell = cast_cell!(LayoutRuleCell);
        cell.set_rule(&self.placer.get_rules()[index]);
        cell.editing_ended.sub(self, move || {
            self.rule_changed.trigger(());
        });
    }
}
