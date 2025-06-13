use std::any::Any;

use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{
        Button, Color, HasText, Label, Setup, TableData, TableView, UI, UIManager, ViewData, cast_cell, view,
    },
    ui_test::record_ui_test,
};

#[view]
struct ScaleView {
    #[init]
    label:  Label,
    button: Button,
    table:  TableView,

    tr_button: Button,
    bl_button: Button,
    br_button: Button,
}

impl Setup for ScaleView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("Label");
        self.label.place().tl(20).size(150, 80);

        self.button.set_text("Button");
        self.button.place().below(self.label, 20);
        self.button.on_tap(move || {
            self.set_color(Color::random());
        });

        self.table.place().size(200, 280).br(20);
        self.table.set_data_source(self);

        self.tr_button.place().tr(20).size(50, 50);
        self.bl_button.place().bl(20).size(50, 50);
        self.br_button.place().br(20).size(50, 50);

        UIManager::set_scale(0.6);
    }
}

impl TableData for ScaleView {
    fn number_of_cells(self: Weak<Self>) -> usize {
        4
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        cast_cell!(Label).set_text(index);
    }
}

pub async fn test_scale() -> Result<()> {
    let _view = UI::init_test_view::<ScaleView>().await;

    record_ui_test().await;

    Ok(())
}
