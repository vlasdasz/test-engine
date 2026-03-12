use anyhow::Result;
use gm::color::{BLACK, TURQUOISE};
use refs::Weak;
use ui::{CellCallbacks, Label, Setup, View, ViewData, ViewFrame, ViewTest, view, view_test};

use crate::{
    self as test_engine,
    ui::{CellRegistry, TableData, TableView},
};

#[view]
struct LoadingCell {
    #[init]
    label: Label,
}

impl Setup for LoadingCell {
    fn setup(self: Weak<Self>) {
        self.label
            .set_color(TURQUOISE)
            .set_text_size(20)
            .set_corner_radius(20)
            .set_border_width(10)
            .set_border_color(BLACK)
            .place()
            .all_sides(5);
    }
}

impl CellCallbacks for LoadingCell {
    fn cell_removed(&mut self) {}
}

#[view_test]
pub struct LoadingCellsTest {
    pub(super) test_string: String,

    #[init]
    pub table: TableView,
}

impl Setup for LoadingCellsTest {
    fn setup(mut self: Weak<Self>) {
        self.table.columns = 2;
        self.table
            .set_data_source(self)
            .register_cell::<LoadingCell>()
            .set_size(400, 400);
        self.table.reload_data();
    }
}

impl TableData for LoadingCellsTest {
    fn cell_height(&self, _index: usize) -> f32 {
        80.0
    }

    fn number_of_cells(&self) -> usize {
        10_000
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        let cell = registry.cell::<LoadingCell>();
        cell.label.set_text(format!("{index} {}", cell.raw().addr()));
        cell
    }

    fn cell_selected(&mut self, index: usize) {
        #[allow(clippy::format_push_string)]
        self.test_string.push_str(&format!("|{index}|"));
    }
}

impl ViewTest for LoadingCellsTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        crate::ui_test::record_ui_test();

        Ok(())
    }
}
