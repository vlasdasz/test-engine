use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::{Own, Weak},
    ui::{GREEN, Label, Setup, TableData, TableView, View, ViewData, ViewFrame, view},
    ui_test::{UITest, check_colors, inject_scroll},
};

#[view]
struct TableViewResize {
    #[init]
    table: TableView,
}

impl Setup for TableViewResize {
    fn setup(self: Weak<Self>) {
        self.table.set_frame((20, 20, 200, 200));
        self.table.set_data_source(self).register_cell::<Label>();
    }
}

impl TableData for TableViewResize {
    fn cell_height(&self, _: usize) -> f32 {
        50.0
    }

    fn number_of_cells(&self) -> usize {
        1
    }

    fn setup_cell(&mut self, _index: usize, registry: &mut test_engine::ui::CellRegistry) -> Own<dyn View> {
        let cell = registry.cell::<Label>();
        cell.set_color(GREEN);
        cell.set_text("alalalalal");
        cell
    }
}

pub async fn test_table_view_resize() -> Result<()> {
    let view = UITest::start::<TableViewResize>();

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
    )?;

    for i in 0..5 {
        inject_scroll(i);
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
    )?;

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
    )?;

    for i in 0..5 {
        inject_scroll(-i);
    }

    Ok(())
}
