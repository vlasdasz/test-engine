use std::{any::Any, ops::Deref};

use anyhow::Result;
use log::debug;
use test_engine::{
    refs::{Own, Weak},
    ui::{
        Button, CollectionData, CollectionView, Container, HasText, Label, Setup, Size, TouchStack, UI, View,
        ViewData, ViewSubviews, view,
    },
    ui_test::{
        inject_touches,
        state::{append_state, get_str_state},
    },
};

#[view]
struct ButtonsOnTableView {
    #[init]
    table: CollectionView,
}

impl Setup for ButtonsOnTableView {
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
        Container::new()
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let cell = cell.downcast_mut::<Container>().unwrap();

        cell.add_view::<Button>()
            .set_image("plus.png")
            .place()
            .size(40, 40)
            .center_y()
            .r(20);

        cell.add_view::<Label>().place().size(100, 40).center_y().l(20);

        cell.get_subview::<Label>().set_text(format!("{index}"));
        cell.get_subview::<Button>().on_tap(move || {
            append_state(format!("button_pressed: {index}\n"));
        });
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (50, 50).into()
    }

    fn cell_selected(&mut self, index: usize) {
        append_state(format!("cell_selected: {index}\n"));
    }
}

pub async fn test_buttons_on_table_view() -> Result<()> {
    UI::init_test_view::<ButtonsOnTableView>().await;

    assert_eq!(TouchStack::dump(), vec![vec![
        "Layer: Root view".to_string(),
        "Container".to_string(),
        "ScrollView.slider: Slider".to_string(),
        "Button".to_string(),
    ]],);

    inject_touches(
        "
            218  146  b
            218  146  e
            218  144  b
            218  144  e
            218  143  b
            218  143  e
            86   26   b
            86   26   e
            85   26   b
            85   26   e
            84   26   b
            84   26   e
            324  36   b
            324  36   e
            324  36   b
            324  36   e
            324  36   b
            324  36   e
            554  34   b
            554  34   e
            554  34   b
            554  34   e
            554  34   b
            554  34   e
        ",
    )
    .await;

    assert_eq!(
        get_str_state(),
        r"cell_selected: 0
cell_selected: 0
cell_selected: 0
cell_selected: 0
cell_selected: 0
cell_selected: 0
button_pressed: 0
button_pressed: 0
button_pressed: 0
"
    );

    debug!("Test buttons on table view: OK");

    Ok(())
}
