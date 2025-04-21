use std::{
    any::Any,
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::Result;
use test_engine::{
    AppRunner, from_main,
    refs::{Own, Weak},
    ui::{
        AfterSetup, Container, GRAY, HasText, Label, Setup, TableData, TableView, UI, View, ViewData,
        ViewSubviews, view,
    },
    ui_test::{helpers::check_colors, inject_touches},
    wait_for_next_frame,
};

static N_CELLS: AtomicUsize = AtomicUsize::new(2_000_000);

#[view]
struct TestTableView {
    #[init]
    table: TableView,
}

impl Setup for TestTableView {
    fn setup(self: Weak<Self>) {
        self.table.place().lr(280).tb(0);
        self.table.set_data_source(self.deref());
    }
}

impl TableData for TestTableView {
    fn cell_height(&self) -> f32 {
        40.0
    }

    fn number_of_cells(&self) -> usize {
        N_CELLS.load(Ordering::Relaxed)
    }

    fn make_cell(&self) -> Own<dyn View> {
        Label::new().after_setup(|mut label| {
            label.add_view::<Container>().set_color(GRAY).place().w(4).sides("tlb", 0);
            label.add_view::<Container>().set_color(GRAY).place().h(4).sides("ltr", 0);
        })
    }

    fn setup_cell(&self, cell: &mut dyn Any, index: usize) {
        let label = cell.downcast_mut::<Label>().unwrap();
        label.set_text(format!("Cell number: {}", index + 1));
    }
}

pub async fn test_table_view() -> Result<()> {
    N_CELLS.store(2_000_000, Ordering::Relaxed);

    let view = UI::init_test_view::<TestTableView>().await;

    AppRunner::set_window_size((1000, 1000)).await;

    wait_for_next_frame().await;
    wait_for_next_frame().await;
    wait_for_next_frame().await;

    assert_eq!(
        view.table.scroll.subviews().last().unwrap().downcast::<Label>().unwrap().text(),
        "Cell number: 26"
    );

    inject_touches(
        "
            865  126  m
            877  111  m
            745  74   m
            680  41   m
            691  28   m
            696  28   m
            699  27   m
            699  27   b
            698  36   m
            697  66   m
            697  123  m
            694  186  m
            693  228  m
            692  272  m
            692  323  m
            693  384  m
            696  422  m
            701  473  m
            707  536  m
            710  653  m
            707  736  m
            698  783  m
            691  820  m
            685  868  m
            678  925  m
            676  959  m
            675  978  m
            674  994  m
            674  1003 m
            672  994  e
            669  983  m
            678  896  m
        ",
    )
    .await;

    assert_eq!(
        view.table.scroll.subviews().last().unwrap().downcast::<Label>().unwrap().text(),
        "Cell number: 2000000"
    );

    from_main(move || {
        N_CELLS.store(2_000_000 - 5, Ordering::Relaxed);
        view.table.reload_data();
    })
    .await;

    check_colors(
        r#"
             242  910 -  89 124 149
             272  888 -  89 124 149
             330  834 - 255 255 255
             331  827 - 255 255 255
             340  779 - 255 255 255
             357  746 -  22  22  22
             358  725 - 219 219 219
             373  693 -   0   0   0
             388  638 - 255 255 255
             413  591 - 255 255 255
             453  569 - 255 255 255
             502  514 - 255 255 255
             513  504 - 255 255 255
             518  467 - 208 208 208
             535  421 - 255 255 255
             548  398 - 255 255 255
             558  390 - 255 255 255
             566  380 - 170 170 170
             575  365 - 255 255 255
             599  344 -  13  13  13
             604  333 - 255 255 255
             616  319 - 255 255 255
             616  248 -  13  13  13
             626  216 - 255 255 255
             658  195 - 255 255 255
             661  191 - 255 255 255
             665  191 - 255 255 255
             704  183 - 255 255 255
             728  181 -  89 124 149
             731  179 -  89 124 149
             871  134 -  89 124 149
        "#,
    )
    .await?;

    assert_eq!(
        view.table.scroll.subviews().last().unwrap().downcast::<Label>().unwrap().text(),
        "Cell number: 1999995"
    );

    Ok(())
}
