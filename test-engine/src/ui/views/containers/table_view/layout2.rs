use gm::LossyConvert;
use refs::{Weak, weak_from_ref};
use ui::{Anchor::Top, ViewData, ViewFrame, ViewSubviews, WeakView};

use crate::ui::{TableView, TableView2};

impl TableView2 {
    pub(super) fn layout_single_column_cells_2(&mut self, number_of_cells: usize) {
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

        let old_cells: Vec<_> = self
            .scroll
            .content
            .subviews_mut()
            .drain(..)
            .map(|c| {
                c.place().clear();
                c
            })
            .collect();

        self.registry.load_old_cells(old_cells);

        let mut weak_table = self.weak();

        for i in first_index..=last_index {
            let cell = self.data.setup_cell2(i, &mut weak_table.registry);
            let cell = self.scroll.add_subview(cell);

            // let cell = self.add_subview(cell);
            cell.place().h(cell_height).t(i.lossy_convert() * cell_height).lr(0);
        }
    }
}

pub(crate) fn layout_two_column_cells_2(table: &mut TableView, number_of_cells: usize) {
    let row_height = table.data.cell_height(0);

    let total_height = (number_of_cells.lossy_convert() / 2.0).ceil() * row_height;

    table.scroll.set_content_height(total_height);

    let mut number_of_cells_fits: usize = (table.height() / row_height).ceil().lossy_convert();
    number_of_cells_fits *= 2;

    let offset = table.scroll.content_offset();

    let mut first_index: usize = (-offset / row_height).floor().lossy_convert();
    if !first_index.is_multiple_of(2) {
        first_index -= 1;
    }
    first_index *= 2;

    let mut last_index = first_index + number_of_cells_fits + 4;

    if last_index > number_of_cells {
        last_index = number_of_cells;
    }

    if first_index == table.first_index && last_index == table.last_index {}

    let h = table.data.cell_height(0);

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
                .custom(move |frame| {
                    frame.origin.x = weak_table.width() / 2.0;
                });
        }
    }
}

pub(crate) fn layout_variable_sized_cells_2(table: &mut TableView, number_of_cells: usize) {
    let mut total_height: f32 = 0.0;

    let mut prev_cell: WeakView = Weak::default();

    for i in 0..number_of_cells {
        let height = table.data.cell_height(i);
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
