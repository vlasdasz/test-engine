use std::any::Any;

use anyhow::Result;
use gm::color::BLACK;
use hreads::{from_main, on_main, sleep, spawn};
use refs::{Own, Weak};
use ui::{Setup, TableData, View, ViewData, ViewTest, cast_cell, view_test};

use crate::{
    self as test_engine,
    ui::{
        Spinner, TableView,
        views::containers::table_view::tests::infinite_scroll::{
            basic_scroll::test_basic_scroll, infinite_cell::InfiniteCell,
        },
    },
    ui_test::{inject_scroll, inject_touches},
};

#[view_test]
pub(super) struct InfiniteScrollTest {
    pub(super) test_string: String,

    data: Vec<usize>,

    requesting: bool,

    #[init]
    table: TableView,
}

impl Setup for InfiniteScrollTest {
    fn setup(mut self: Weak<Self>) {
        self.table.columns = 2;
        self.table
            .set_data_source(self)
            .set_border_color(BLACK)
            .set_border_width(5)
            .place()
            .t(120)
            .b(120)
            .lr(0);

        self.table.bottom_reached().sub(self, move || {
            if self.requesting {
                return;
            }

            self.requesting = true;

            spawn(async move {
                self.on_fetch().await;
            });
        });

        self.data = (0..=199).collect();
    }
}

impl InfiniteScrollTest {
    async fn on_fetch(mut self: Weak<Self>) {
        let _spin = Spinner::lock();

        let data = fetch_more_data(self.data.len()).await;

        on_main(move || {
            self.data.extend(data);
            self.table.reload_data();
            self.requesting = false;
        });
    }
}

impl TableData for InfiniteScrollTest {
    fn cell_height(self: Weak<Self>, _index: usize) -> f32 {
        80.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        self.data.len()
    }

    fn make_cell(self: Weak<Self>, _index: usize) -> Own<dyn View> {
        InfiniteCell::new()
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        cast_cell!(InfiniteCell).set_text(self.data[index]);
    }

    fn cell_selected(mut self: Weak<Self>, index: usize) {
        #[allow(clippy::format_push_string)]
        self.test_string.push_str(&format!("|{index}|"));
    }
}

impl ViewTest for InfiniteScrollTest {
    fn perform_test(mut view: Weak<Self>) -> Result<()> {
        test_basic_scroll(view)?;

        inject_scroll(-8000);

        for _ in 0..20 {
            inject_scroll(-10);
        }

        #[allow(clippy::while_immutable_condition)]
        while view.requesting {}

        for _ in 0..100 {
            inject_scroll(-10);
        }

        from_main(move || {
            view.test_string.clear();
        });

        inject_touches(
            "
                 186  181  b
                 186  181  e
                 398  190  b
                 398  190  e
                 387  264  b
                 387  264  e
                 204  262  b
                 204  262  e
                 196  334  b
                 195  334  e
                 427  362  b
                 427  362  e
                 419  412  b
                 419  412  e
                 176  396  b
                 176  396  e
             ",
        );

        assert_eq!(view.test_string, "|216||217||219||218||220||223||223||222|");

        // crate::ui_test::record_ui_test();

        Ok(())
    }
}

async fn fetch_more_data(last_index: usize) -> Vec<usize> {
    sleep(0.5).await;

    (last_index..last_index + 100).collect()
}
