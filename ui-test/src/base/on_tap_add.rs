use std::{any::Any, ops::Deref};

use anyhow::Result;
use test_engine::{
    refs::{Own, Weak},
    ui::{
        Button, CollectionData, CollectionView, Color, Container, HasText, ImageView, Label, Setup, Size, UI,
        View, ViewData, ViewSubviews,
        ui_test::{helpers::check_colors, inject_touches},
        view,
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

impl Setup for SomeView {
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

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        cell.downcast_mut::<Label>().unwrap().set_text(format!("{index}"));
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (50, 50).into()
    }

    fn make_cell(&self) -> Own<dyn View> {
        Label::new()
    }
}

#[view]
struct AddOnTap {
    #[init]
    btn: Button,
}

impl Setup for AddOnTap {
    fn setup(mut self: Weak<Self>) {
        self.btn.set_text("A").place().size(50, 50);
        self.btn.on_tap(move || {
            let view = self.add_view::<SomeView>();
            view.place().size(600, 500).br(5);
        });
    }
}

pub async fn test_add_on_tap() -> Result<()> {
    let view = UI::init_test_view::<AddOnTap>().await;

    assert_eq!(view.dump_subviews(), vec!["AddOnTap.btn: Button".to_string()]);

    inject_touches(
        "
            25   25   b
            25   25   e
        ",
    )
    .await;

    assert_eq!(
        view.dump_subviews(),
        vec!["AddOnTap.btn: Button".to_string(), "SomeView".to_string()]
    );

    check_colors(
        r#"
              20  569 -  89 124 149
              36  540 -  33 150 243
              50  525 -  33 150 243
              72  500 -  33 150 243
              98  476 - 250 250 250
             118  458 -  33 150 243
             139  441 -  33 150 243
             164  417 -  33 150 243
             222  386 -  89 124 149
             306  388 -  89 124 149
             335  402 -  89 124 149
             408  418 -   0 255 255
             478  411 -   0 255 255
             483  398 -   0 255 255
             491  285 -   0 255   0
             493  217 -   0 255   0
             522  187 -   0 255   0
             466  201 -   0   0   0
             452  204 -   0   0   0
             536  155 -   0 255   0
             499  157 -   0 255   0
             367  149 - 255 255 255
             255  165 - 255 255 255
             222  172 - 255 255 255
             175  166 - 255 255 255
             199  139 - 255 255 255
             196  103 - 255 255 255
             154   85 -  89 124 149
             111   64 -  89 124 149
              85   46 -  89 124 149
              64   35 -  89 124 149
              53   31 -  89 124 149
              41   25 - 255 255 255
              31   24 -  73  73  73
              18   24 - 134 134 134
              71   28 -  89 124 149
             165   87 -  89 124 149
             299  189 - 255 255 255
             215  354 -  89 124 149
             449  216 -   0 255   0
             503  106 -   0 255   0
        "#,
    )
    .await?;

    Ok(())
}
