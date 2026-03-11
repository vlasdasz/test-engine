use gm::LossyConvert;
use refs::weak_from_ref;
use ui::{ViewData, ViewFrame, ViewSubviews, ViewTouch};

use crate::ui::TableView;

impl TableView {
    fn clear_old_cells(&mut self) {
        let old_cells: Vec<_> = self.scroll.content.subviews_mut().drain(..).collect();

        self.registry.load_old_cells(old_cells);
    }

    pub(super) fn layout_single_column_cells(&mut self, number_of_cells: usize) {
        let cell_height = self.data.cell_height(0);

        let total_height = number_of_cells.lossy_convert() * cell_height;

        let width = self.width();

        self.scroll.set_content_height(total_height);
        self.scroll.set_content_width(width);

        let number_of_cells_fits: usize = (self.height() / cell_height).ceil().lossy_convert();

        let offset = self.scroll.get_scroll_content_offset();

        let first_index: usize = (-offset / cell_height).floor().lossy_convert();

        let mut last_index = first_index + number_of_cells_fits + 1;

        if last_index > number_of_cells {
            last_index = number_of_cells;
        }

        self.clear_old_cells();

        let mut weak_table = self.weak();

        for i in first_index..last_index {
            let cell = self.data.setup_cell(i, &mut weak_table.registry);

            cell.set_frame((0, i.lossy_convert() * cell_height, width, cell_height));

            cell.enable_touch_low_priority();
            cell.touch().up_inside.sub(weak_table, move || {
                weak_table.data.cell_selected(i);
            });
        }
    }

    pub(crate) fn layout_two_column_cells(&mut self, number_of_cells: usize) {
        let row_height = self.data.cell_height(0);
        let cell_width = self.width() / 2.0;

        let total_height = (number_of_cells.lossy_convert() / 2.0).ceil() * row_height;

        self.scroll.set_content_height(total_height);

        let mut number_of_cells_fits: usize = (self.height() / row_height).ceil().lossy_convert();
        number_of_cells_fits *= 2;

        let offset = self.scroll.get_scroll_content_offset();

        let mut first_index: usize = (-offset / row_height).floor().lossy_convert();
        if !first_index.is_multiple_of(2) {
            first_index -= 1;
        }
        first_index *= 2;

        let mut last_index = first_index + number_of_cells_fits + 4;

        if last_index > number_of_cells {
            last_index = number_of_cells;
        }

        self.clear_old_cells();

        let mut weak_table = weak_from_ref(self);

        for i in first_index..last_index {
            let cell = self.data.setup_cell(i, &mut weak_table.registry);

            cell.enable_touch_low_priority();
            cell.touch().up_inside.sub(weak_table, move || {
                weak_table.data.cell_selected(i);
            });

            let y_pos = (i / 2).lossy_convert() * row_height;

            if i % 2 == 0 {
                cell.set_frame((0, y_pos, cell_width, row_height));
            } else {
                cell.set_frame((cell_width, y_pos, cell_width, row_height));
            }
        }
    }
}
