use std::any::Any;

use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::{Own, Weak},
    ui::{
        AfterSetup, GREEN, HasText, Label, Setup, TableData, TableView, UI, View, ViewData, ViewFrame, view,
    },
    ui_test::{check_colors, inject_scroll},
};

#[view]
struct TableViewResize {
    #[init]
    table: TableView,
}

impl Setup for TableViewResize {
    fn setup(mut self: Weak<Self>) {
        self.table.set_frame((20, 20, 200, 200));
        self.table.set_data_source(self);
    }
}

impl TableData for TableViewResize {
    fn cell_height(self: Weak<Self>) -> f32 {
        50.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        1
    }

    fn make_cell(self: Weak<Self>, _index: usize) -> Own<dyn View> {
        Label::new().after_setup(|mut label| {
            label.set_color(GREEN);
        })
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, _index: usize) {
        let label = cell.downcast_mut::<Label>().unwrap();
        label.set_text("alalalalal");
    }
}

pub async fn test_table_view_resize() -> Result<()> {
    let mut view = UI::init_test_view::<TableViewResize>().await;

    check_colors(
        r#"
              11   40 -  89 124 149
              33   39 -   0 255   0
             199   36 -   0 255   0
             245   40 -  89 124 149
             173  100 -  89 124 149
             191   55 -   0 255   0
             164   11 -  89 124 149
              42   49 -   0 255   0
              42   91 -  89 124 149
        "#,
    )
    .await?;

    for i in 0..5 {
        inject_scroll(i).await;
    }

    check_colors(
        r#"
              11   40 -  89 124 149
              33   39 -   0 255   0
             199   36 -   0 255   0
             245   40 -  89 124 149
             173  100 -  89 124 149
             191   55 -   0 255   0
             164   11 -  89 124 149
              42   49 -   0 255   0
              42   91 -  89 124 149
        "#,
    )
    .await?;

    from_main(move || {
        view.table.set_size(400, 100);
    });

    check_colors(
        r#"
               9   42 -  89 124 149
              36   43 -   0 255   0
             128   43 -   0 255   0
             140   82 -  89 124 149
             143    9 -  89 124 149
             385   45 -   0 255   0
             428   44 -  89 124 149
             442   44 -  89 124 149
             362   97 -  89 124 149
             355   10 -  89 124 149
        "#,
    )
    .await?;

    for i in 0..5 {
        inject_scroll(-i).await;
    }

    Ok(())
}
