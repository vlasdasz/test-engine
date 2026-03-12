use gm::LossyConvert;
use refs::weak_from_ref;
use ui::{ViewData, ViewFrame, ViewSubviews, ViewTouch};

use crate::ui::TableView;

impl TableView {
    fn clear_old_cells(&mut self) {
        let old_cells: Vec<_> = self
            .scroll
            .content
            .subviews_mut()
            .iter()
            .map(|c| {
                let mut weak = c.weak_view();
                weak.set_hidden(true);
                weak.as_cell().cell_removed();
                weak
            })
            .collect();

        self.registry.load_old_cells(old_cells);
    }

    pub(super) fn layout_fixed_cells(&mut self, number_of_cells: usize, columns: usize) {
        let cell_height = self.data.cell_height(0);
        let width = self.width();
        let cell_width = width / columns.lossy_convert();

        let total_height =
            (number_of_cells.lossy_convert() / columns.lossy_convert()).ceil() * cell_height;

        self.scroll.set_content_height(total_height);
        if columns == 1 {
            self.scroll.set_content_width(width);
        }

        let rows_fit: usize = (self.height() / cell_height).ceil().lossy_convert();
        let number_of_cells_fits = rows_fit * columns;

        let offset = self.scroll.get_scroll_content_offset();

        let first_visible_row: usize = (-offset / cell_height).floor().lossy_convert();
        let first_index = first_visible_row * columns;

        let mut last_index = first_index + number_of_cells_fits + columns * 2;
        if last_index > number_of_cells {
            last_index = number_of_cells;
        }

        self.clear_old_cells();

        let mut weak_table = weak_from_ref(self);

        for i in first_index..last_index {
            let cell = self.data.setup_cell(i, &mut weak_table.registry);

            let x: f32 = (i % columns).lossy_convert() * cell_width;
            let y: f32 = (i / columns).lossy_convert() * cell_height;
            cell.set_frame((x, y, cell_width, cell_height));

            cell.enable_touch_low_priority();
            cell.touch().up_inside.sub(weak_table, move || {
                weak_table.data.cell_selected(i);
            });
        }
    }
}
