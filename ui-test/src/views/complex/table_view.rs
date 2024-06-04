use std::{
    any::Any,
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::{Own, Weak},
    ui::{
        view, AfterSetup, Color, Container, Label, TableData, TableView, View, ViewData, ViewSetup,
        ViewSubviews, UI,
    },
    ui_test::{helpers::check_colors, inject_touches},
    wait_for_next_frame, App,
};

static N_CELLS: AtomicUsize = AtomicUsize::new(2_000_000);

#[view]
struct TestTableView {
    #[init]
    table: TableView,
}

impl ViewSetup for TestTableView {
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

    App::set_window_size((1000, 1000)).await;

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
        r"
             666  983 - 255 255 255
             625  983 -  59  59  59
             591  983 - 255 255 255
             511  983 - 255 255 255
             477  983 - 255 255 255
             441  983 - 255 255 255
             389  983 - 255 255 255
             367  973 - 255 255 255
             351  973 - 255 255 255
             293  976 - 255 255 255
             252  976 -  25  51  76
             743  977 -  25  51  76
        ",
    )
    .await?;

    assert_eq!(
        view.table.scroll.subviews().last().unwrap().downcast::<Label>().unwrap().text(),
        "Cell number: 1999995"
    );

    debug!("Table view test: OK");

    Ok(())
}
