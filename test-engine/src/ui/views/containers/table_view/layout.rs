use gm::LossyConvert;
use refs::{Weak, weak_from_ref};
use ui::{Anchor::Top, ViewData, ViewFrame, WeakView};

use crate::ui::TableView;

pub(super) fn layout_single_column_cells(table: &mut TableView, number_of_cells: usize) {
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

pub(super) fn layout_two_column_cells(table: &mut TableView, number_of_cells: usize) {
    let row_height = table.data.__cell_height(0);

    let total_height = (number_of_cells.lossy_convert() / 2.0).ceil() * row_height;

    table.scroll.set_content_height(total_height);

    let mut number_of_cells_fits: usize = (table.height() / row_height).ceil().lossy_convert();
    number_of_cells_fits *= 2;

    let offset = table.scroll.content_offset();

    let mut first_index: usize = (-offset / row_height).floor().lossy_convert();
    first_index /= 2;

    let mut last_index = first_index + number_of_cells_fits + 4;

    if last_index > number_of_cells {
        last_index = number_of_cells;
    }

    let h = table.data.__cell_height(0);

    let weak_table = weak_from_ref(table);

    for i in first_index..last_index {
        if i % 2 == 0 {
            table
                .add_cell(i)
                .place()
                .h(h)
                .t((i / 2).lossy_convert() * h)
                .l(0)
                .relative_width(weak_table, 0.5);
        } else {
            table
                .add_cell(i)
                .place()
                .h(h)
                .relative_width(weak_table, 0.5)
                .t((i / 2).lossy_convert() * h)
                .custom(move |mut view| {
                    view.set_x(weak_table.width() / 2.0);
                });
        }
    }
}

pub(super) fn layout_variable_sized_cells(table: &mut TableView, number_of_cells: usize) {
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
