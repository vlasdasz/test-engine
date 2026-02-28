use gm::LossyConvert;
use refs::Weak;
use ui::{Anchor::Top, ViewData, ViewFrame, WeakView};

use crate::ui::TableView;

pub(super) fn layout_same_sized_cells(mut table: Weak<TableView>, number_of_cells: usize) {
    let cell_height = table.data.__cell_height(0);

    let total_height = number_of_cells.lossy_convert() * cell_height;

    table.scroll.set_content_height(total_height);

    let number_of_cells_fits: usize = (table.height() / cell_height).ceil().lossy_convert();

    let offset = table.scroll.content_offset();

    let first_index: usize = (-offset / cell_height).floor().lossy_convert();

    let mut last_index = first_index + number_of_cells_fits + 1;

    if last_index > number_of_cells {
        last_index = number_of_cells;
    }

    let h = table.data.__cell_height(0);

    for i in first_index..last_index {
        table.add_cell(i).place().h(h).t(i.lossy_convert() * h).lr(0);
    }
}

pub(super) fn layout_variable_sized_cells(mut table: Weak<TableView>, number_of_cells: usize) {
    let mut total_height: f32 = 0.0;

    let mut prev_cell: WeakView = Weak::default();

    for i in 0..number_of_cells {
        let height = table.data.__cell_height(i);
        total_height += height;

        let cell = table.add_cell(i);

        cell.place().lr(0).h(height);

        if i == 0 {
            cell.place().t(0);
        } else {
            cell.place().anchor(Top, prev_cell, 0);
        }

        prev_cell = cell;
    }

    table.scroll.set_content_height(total_height);
}
