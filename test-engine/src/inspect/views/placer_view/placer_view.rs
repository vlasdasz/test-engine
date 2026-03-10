use refs::{Own, Rglica, ToRglica, Weak};
use ui::{AfterSetup, Placer, Setup, UIEvent, View, ViewData};
use ui_proc::view;

use crate::{
    inspect::views::LayoutRuleCell,
    ui::{CellRegistry, TableData, TableView},
};

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
        self.table.set_data_source(self).register_cell::<LayoutRuleCell>();
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
    fn cell_height(&self, _: usize) -> f32 {
        50.0
    }

    fn number_of_cells(&self) -> usize {
        if self.placer.is_null() {
            return 0;
        }
        self.placer.get_rules().len()
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Own<dyn View> {
        let this = self.weak();
        registry.get_cell::<LayoutRuleCell>().after_setup(move |cell| {
            if this.placer.is_null() {
                return;
            }
            cell.set_rule(&this.placer.get_rules()[index]);
            cell.editing_ended.sub(this, move || {
                this.rule_changed.trigger(());
            });
        })
    }
}
