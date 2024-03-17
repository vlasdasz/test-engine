use std::{any::Any, ops::Deref};

use anyhow::Result;
use test_engine::{
    refs::{Own, Weak},
    ui::{view, Color, Container, Label, Sub, TableData, TableView, View, ViewData, ViewSetup, ViewSubviews},
    ui_test::record_ui_test,
    App,
};

#[view]
struct TestTableView {
    table: Sub<TableView>,
}

impl ViewSetup for TestTableView {
    fn setup(self: Weak<Self>) {
        self.table.place().all_sides(150);
        self.table.set_data_source(self.deref());
        self.table.layout_cells();
    }
}

impl TableData for TestTableView {
    fn cell_height(&self) -> f32 {
        40.0
    }

    fn number_of_cells(&self) -> usize {
        500
    }

    fn make_cell(&self) -> Own<dyn View> {
        Label::new()
    }

    fn setup_cell(&self, cell: &mut dyn Any) {
        let label = cell.downcast_mut::<Label>().unwrap();

        label
            .add_view::<Container>()
            .set_color(Color::GRAY)
            .place()
            .w(4)
            .sides("tlb", 0);
        label
            .add_view::<Container>()
            .set_color(Color::GRAY)
            .place()
            .h(4)
            .sides("ltr", 0);
    }

    fn setup_cell_for_reuse(&self, cell: &mut dyn Any, index: usize) {
        let label = cell.downcast_mut::<Label>().unwrap();
        label.set_text(format!("Cell number: {index}"));
    }
}

pub async fn test_table_view() -> Result<()> {
    let mut _view = App::init_test_view::<TestTableView>().await;

    App::set_window_size((1000, 1000));

    record_ui_test().await;

    Ok(())
}
