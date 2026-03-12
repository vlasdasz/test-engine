use gm::LossyConvert;
use refs::weak_from_ref;
use ui::{ViewData, ViewFrame, ViewSubviews, ViewTouch};

use crate::ui::TableView;

impl TableView {
    pub(super) fn layout_fixed_cells(&mut self, number_of_cells: usize, columns: usize, force: bool) {
        let cell_height = self.data.cell_height(0);
        let width = self.width();
        let cell_width = width / columns.lossy_convert();

        let total_height = (number_of_cells.lossy_convert() / columns.lossy_convert()).ceil() * cell_height;

        self.scroll.set_content_height(total_height);
        if columns == 1 {
            self.scroll.set_content_width(width);
        }

        let rows_fit: usize = (self.height() / cell_height).ceil().lossy_convert();
        let offset = self.scroll.get_scroll_content_offset();
        let first_visible_row: usize = (-offset / cell_height).floor().lossy_convert();
        let first_index = first_visible_row * columns;

        let mut last_index = first_index + rows_fit * columns + columns * 2;
        if last_index > number_of_cells {
            last_index = number_of_cells;
        }

        let mut to_recycle = Vec::new();
        let mut shown = Vec::new();

        for subview in self.scroll.content.subviews_mut().iter() {
            let weak = subview.weak_view();
            if weak.is_hidden() {
                continue;
            }
            let idx = weak.tag();
            if !force && idx >= first_index && idx < last_index {
                shown.push(idx);
            } else {
                to_recycle.push(weak);
            }
        }

        let recycled: Vec<_> = to_recycle
            .into_iter()
            .map(|mut cell| {
                cell.set_hidden(true);
                cell.as_cell().cell_removed();
                cell
            })
            .collect();

        self.registry.load_old_cells(recycled);

        let mut weak_table = weak_from_ref(self);

        for i in first_index..last_index {
            if shown.contains(&i) {
                continue;
            }

            let mut cell = self.data.setup_cell(i, &mut weak_table.registry);
            cell.set_tag(i);

            let x: f32 = (i % columns).lossy_convert() * cell_width;
            let y: f32 = (i / columns).lossy_convert() * cell_height;
            cell.set_frame((x, y, cell_width, cell_height));

            cell.touch().up_inside.clear_subscribers();
            cell.enable_touch_low_priority();
            cell.touch().up_inside.sub(weak_table, move || {
                weak_table.data.cell_selected(i);
            });
        }
    }
}
