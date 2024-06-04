use std::{any::Any, ops::Deref};

use anyhow::Result;
use log::debug;
use test_engine::{
    refs::{Own, Weak},
    ui::{
        ui_test::{helpers::check_colors, inject_touches},
        view, Button, CollectionData, CollectionView, Color, Container, ImageView, Label, Size, View,
        ViewData, ViewSetup, ViewSubviews, UI,
    },
};

#[view]
struct SomeView {
    #[init]
    table:  CollectionView,
    label:  Label,
    image:  ImageView,
    square: Container,
}

impl ViewSetup for SomeView {
    fn setup(mut self: Weak<Self>) {
        self.table.set_data_source(self.deref()).place().size(400, 400);
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
        2
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
struct OnTapAddTestView {
    #[init]
    btn: Button,
}

impl ViewSetup for OnTapAddTestView {
    fn setup(mut self: Weak<Self>) {
        self.btn.set_text("A").place().size(50, 50);
        self.btn.on_tap(move || {
            let view = self.add_view::<SomeView>();
            view.place().size(600, 500).br(5);
        });
    }
}

pub async fn test_add_on_tap() -> Result<()> {
    let view = UI::init_test_view::<OnTapAddTestView>().await;

    assert_eq!(
        view.dump_subviews(),
        vec!["OnTapAddTestView.btn: Button".to_string()]
    );

    inject_touches(
        "
            25   25   b
            25   25   e
        ",
    )
    .await;

    assert_eq!(
        view.dump_subviews(),
        vec!["OnTapAddTestView.btn: Button".to_string(), "SomeView".to_string()]
    );

    check_colors(
        r"
              19  561 -  25  51  76
              42  551 -   3  77 228
              89  521 -   3  77 228
             154  469 -   3  77 228
             164  455 -   3  77 228
             132  452 -   3  77 228
             156  397 -  25  51  76
             155  254 -  25  51  76
             191  210 -  25  51  76
             205  167 - 255 255 255
             203  151 - 255 255 255
             198  165 - 255 255 255
             179  167 - 255 255 255
             185  127 - 255 255 255
             193  109 - 232 232 232
             266   27 -  25  51  76
             345   64 -  25  51  76
             362   87 -  25  51  76
             413  148 -   0 255   0
             497  179 -   0 255   0
             514  122 -   0 255   0
             521   78 -  25  51  76
             545   97 -  25  51  76
             529  150 -   0 255   0
             483  217 -   0 255   0
             463  302 -   0 255   0
             486  435 -   0 255 255
             498  489 -   0 255 255
             389  539 -   0 255 255
             281  446 -  25  51  76
             397  369 -  25  51  76
             482  310 -  25  51  76
             446  211 -   0 255   0
             266  126 - 255 255 255
             150  109 - 255 255 255
             115  114 - 255 255 255
             111   82 -  25  51  76
             176   41 -  25  51  76
             237   37 -  25  51  76
        ",
    )
    .await?;

    debug!("Add on tap: OK");

    Ok(())
}
