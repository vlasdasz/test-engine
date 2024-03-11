use std::{any::Any, ops::Deref};

use anyhow::Result;
use log::debug;
use test_engine::{
    refs::{Own, Weak},
    ui::{
        view, Button, CollectionData, CollectionView, Container, Label, Size, Sub, View, ViewData, ViewSetup,
        ViewSubviews,
    },
    ui_test::record_ui_test,
    App,
};

#[view]
struct ButtonsOnTableView {
    table: Sub<CollectionView>,
}

impl ViewSetup for ButtonsOnTableView {
    fn setup(self: Weak<Self>) {
        self.table.place().back();
        self.table.set_data_source(self.deref());
    }
}

impl CollectionData for ButtonsOnTableView {
    fn number_of_cells(&self) -> usize {
        1
    }

    fn make_cell(&self) -> Own<dyn View> {
        let mut view = Container::new();

        view.add_view::<Button>()
            .set_image("plus.png")
            .place()
            .size(40, 40)
            .center_y()
            .r(20);

        view.add_view::<Label>().place().size(100, 40).center_y().l(20);

        view
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let cell = cell.downcast_mut::<Container>().unwrap();
        cell.get_subview::<Label>().set_text(format!("{index}"));
        cell.get_subview::<Button>().on_tap(move || {
            dbg!(&index);
        });
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (50, 50).into()
    }

    fn cell_selected(&mut self, _index: usize) {
        dbg!(_index);
    }
}

pub async fn test_buttons_on_table_view() -> Result<()> {
    App::init_test_view::<ButtonsOnTableView>().await;

    record_ui_test().await;

    debug!("Test buttons on table view: OK");

    Ok(())
}
