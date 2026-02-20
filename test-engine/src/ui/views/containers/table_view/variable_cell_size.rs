use anyhow::Result;
use gm::color::BLACK;
use refs::{Own, Weak};
use ui::{Container, Setup, TableData, View, ViewData, ViewTest, view_test};

use crate as test_engine;
use crate::{ui::TableView, ui_test::check_colors};

#[view_test]
struct VariableCellSizeTest {
    #[init]
    table: TableView,
}

impl Setup for VariableCellSizeTest {
    fn setup(self: Weak<Self>) {
        // UIManager::enable_debug_frames();
        self.table.set_data_source(self).place().back();
        self.table.reload_data();
    }
}

impl TableData for VariableCellSizeTest {
    fn cell_height(self: Weak<Self>, index: usize) -> f32 {
        [40.0, 200.0, 80.0, 100.0][index]
    }

    fn variable_height(self: Weak<Self>) -> bool {
        true
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        4
    }

    fn make_cell(self: Weak<Self>, _index: usize) -> Own<dyn View> {
        let cell = Container::new();

        cell.weak().set_border_color(BLACK).set_border_width(20).set_corner_radius(10);

        cell
    }
}

impl ViewTest for VariableCellSizeTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
                      38  431 -  89 124 149
                      40  416 -   0   0   0
                      42  376 -  89 124 149
                      44  345 -  89 124 149
                      46  324 -   0   0   0
                      47  318 -   0   0   0
                      48  311 -   0   0   0
                      48  294 -  89 124 149
                      49  259 -  89 124 149
                      50  243 -   0   0   0
                      50  223 -  89 124 149
                      50  183 -  89 124 149
                      50  130 -  89 124 149
                      55   99 -  89 124 149
                      55   66 -  89 124 149
                      58   53 -  89 124 149
                      58   43 -   0   0   0
                      59   25 -  89 124 149
                      58   10 -  89 124 149
                ",
        )?;

        // crate::ui_test::record_ui_test();
        Ok(())
    }
}
