use std::{any::Any, ops::Deref};

use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::{Own, Weak},
    ui::{
        view, CollectionData, CollectionView, Color, Container, ImageView, Label, Size, Sub, View, ViewData,
        ViewSetup,
    },
    ui_test::record_ui_test,
    App,
};

#[view]
struct SomeView {
    table:  Sub<CollectionView>,
    label:  Sub<Label>,
    image:  Sub<ImageView>,
    square: Sub<Container>,
}

impl ViewSetup for SomeView {
    fn setup(mut self: Weak<Self>) {
        self.table.set_data_source(self.deref()).place().size(400, 400);
        self.table.reload_data();
        self.label
            .set_text("Hello")
            .set_color(Color::GREEN)
            .place()
            .size(200, 200)
            .tr(10);
        self.image.set_image("plus.png").place().size(200, 200).bl(10);
        self.square.set_color(Color::TURQUOISE).place().size(200, 200).br(10);
    }
}

impl CollectionData for SomeView {
    fn number_of_cells(&self) -> usize {
        5
    }

    fn make_cell(&self) -> Own<dyn View> {
        Label::new()
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        cell.downcast_mut::<Label>().unwrap().set_text(format!("{index}"));
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (50, 50).into()
    }
}

#[view]
struct OnTapAddTestView {}

pub async fn test_add_on_tap() -> Result<()> {
    let mut view = App::init_test_view::<SomeView>().await;

    from_main(move || {
        view.table.reload_data();
    })
    .await;

    record_ui_test().await;

    debug!("Add on tap: OK");

    Ok(())
}
