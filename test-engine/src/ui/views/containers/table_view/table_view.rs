use gm::LossyConvert;
use refs::Weak;
use ui::{
    __ViewInternalTableData, Anchor::Top, ScrollView, Setup, ViewCallbacks, ViewData, ViewFrame,
    ViewSubviews, ViewTouch, WeakView, view,
};

use crate as test_engine;

#[view]
pub struct TableView {
    data: Weak<dyn __ViewInternalTableData>,

    #[init]
    pub scroll: ScrollView,
}

impl ViewCallbacks for TableView {
    fn update(&mut self) {
        let width = self.width();
        self.scroll.set_content_width(width);
    }
}

impl Setup for TableView {
    fn setup(self: Weak<Self>) {
        self.scroll.place().back();
        self.scroll.on_scroll.sub(move || {
            self.layout_cells();
        });

        self.size_changed().sub(move || {
            self.layout_cells();
        });
    }
}

impl TableView {
    pub fn set_data_source<T: __ViewInternalTableData + 'static>(
        mut self: Weak<Self>,
        data: Weak<T>,
    ) -> Weak<Self> {
        self.data = data;
        self
    }

    pub fn reload_data(self: Weak<Self>) {
        self.layout_cells();
    }
}

impl TableView {
    fn layout_cells(self: Weak<Self>) {
        self.scroll.remove_all_subviews();

        if self.height() <= 0.0 {
            return;
        }

        assert!(
            self.data.is_ok(),
            "TableView data source is not set. Use set_data_source method."
        );

        let number_of_cells = self.data.__number_of_cells();

        if number_of_cells == 0 {
            return;
        }

        if self.data.__variable_height() {
            self.layout_variable_sized_cells(number_of_cells);
        } else {
            self.layout_same_sized_cells(number_of_cells);
        }
    }

    fn add_cell(mut self: Weak<Self>, index: usize) -> WeakView {
        let mut cell = self.data.__make_cell(index);

        let label = format!("TableView cell: {}", cell.label());
        cell.set_label(label);

        let mut cell = self.scroll.add_subview(cell);

        self.data.__setup_cell(cell.as_any_mut(), index);

        cell.enable_touch_low_priority();
        cell.touch().up_inside.sub(move || {
            self.data.__cell_selected(index);
        });

        cell
    }

    fn layout_same_sized_cells(mut self: Weak<Self>, number_of_cells: usize) {
        let cell_height = self.data.__cell_height(0);

        let total_height = number_of_cells.lossy_convert() * cell_height;

        self.scroll.set_content_height(total_height);

        let number_of_cells_fits: usize = (self.height() / cell_height).ceil().lossy_convert();

        let offset = self.scroll.content_offset();

        let first_index: usize = (-offset / cell_height).floor().lossy_convert();

        let mut last_index = first_index + number_of_cells_fits + 1;

        if last_index > number_of_cells {
            last_index = number_of_cells;
        }

        let h = self.data.__cell_height(0);

        for i in first_index..last_index {
            self.add_cell(i).place().h(h).t(i.lossy_convert() * h).lr(0);
        }
    }

    fn layout_variable_sized_cells(mut self: Weak<Self>, number_of_cells: usize) {
        let mut total_height: f32 = 0.0;

        let mut prev_cell: WeakView = Weak::default();

        for i in 0..number_of_cells {
            let height = self.data.__cell_height(i);
            total_height += height;

            let cell = self.add_cell(i);

            cell.place().lr(0).h(height);

            if i == 0 {
                cell.place().t(0);
            } else {
                cell.place().anchor(Top, prev_cell, 0);
            }

            prev_cell = cell;
        }

        self.scroll.set_content_height(total_height);
    }
}

mod test {
    use std::{
        any::Any,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use anyhow::Result;
    use gm::color::GRAY;
    use hreads::{from_main, wait_for_next_frame};
    use refs::{Own, Weak};
    use ui::{
        AfterSetup, Container, Label, Setup, TableData, View, ViewData, ViewSubviews, ViewTest, view_test,
    };

    use crate::{
        self as test_engine, AppRunner,
        ui::TableView,
        ui_test::{UITest, check_colors, inject_touches},
    };

    static N_CELLS: AtomicUsize = AtomicUsize::new(2_000_000);
    static INDEX: AtomicUsize = AtomicUsize::new(0);

    #[view_test]
    struct TestTableView {
        #[init]
        table: TableView,
    }

    impl Setup for TestTableView {
        fn setup(self: Weak<Self>) {
            self.table.place().lr(280).tb(0);
            self.table.set_data_source(self);
        }
    }

    impl TableData for TestTableView {
        fn cell_height(self: Weak<Self>, _: usize) -> f32 {
            40.0
        }

        fn number_of_cells(self: Weak<Self>) -> usize {
            N_CELLS.load(Ordering::Relaxed)
        }

        fn make_cell(self: Weak<Self>, _index: usize) -> Own<dyn View> {
            Label::new().after_setup(|label| {
                label.add_view::<Container>().set_color(GRAY).place().w(4).sides("tlb", 0);
                label.add_view::<Container>().set_color(GRAY).place().h(4).sides("ltr", 0);
            })
        }

        fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
            let label = cell.downcast_mut::<Label>().unwrap();
            label.set_text(format!("Cell number: {}", index + 1));
        }

        fn cell_selected(self: Weak<Self>, index: usize) {
            INDEX.store(index, Ordering::Relaxed);
        }
    }

    impl ViewTest for TestTableView {
        #[allow(clippy::too_many_lines)]
        fn perform_test(_view: Weak<Self>) -> Result<()> {
            N_CELLS.store(2_000_000, Ordering::Relaxed);

            let view = UITest::start::<TestTableView>();

            AppRunner::set_window_size((1000, 1000));

            wait_for_next_frame();
            wait_for_next_frame();
            wait_for_next_frame();

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
            );

            assert_eq!(
                view.table.scroll.subviews().last().unwrap().downcast::<Label>().unwrap().text(),
                "Cell number: 2000000"
            );

            from_main(move || {
                N_CELLS.store(2_000_000 - 5, Ordering::Relaxed);
                view.table.reload_data();
            });

            check_colors(
                r"
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
                ",
            )?;

            assert_eq!(
                view.table.scroll.subviews().last().unwrap().downcast::<Label>().unwrap().text(),
                "Cell number: 1999995"
            );

            inject_touches(
                "
                    704  979  b
                    703  976  m
                    700  966  m
                    698  952  m
                    694  939  m
                    693  925  m
                    692  913  m
                    692  899  m
                    692  883  m
                    692  868  m
                    692  849  m
                    692  830  m
                    693  814  m
                    693  797  m
                    693  782  m
                    693  763  m
                    695  747  m
                    695  732  m
                    695  718  m
                    697  705  m
                    697  692  m
                    697  676  m
                    699  661  m
                    699  642  m
                    699  623  m
                    699  600  m
                    700  579  m
                    700  559  m
                    700  531  m
                    700  504  m
                    700  475  m
                    700  451  m
                    699  427  m
                    699  401  m
                    699  377  m
                    697  353  m
                    697  329  m
                    697  300  m
                    697  272  m
                    695  245  m
                    693  223  m
                    691  202  m
                    687  187  m
                    685  177  m
                    681  187  m
                    677  218  m
                    675  256  m
                    671  302  m
                    667  344  m
                    665  386  m
                    663  415  m
                    661  441  m
                    661  463  m
                    660  484  m
                    658  503  m
                    658  518  m
                    659  514  m
                    662  493  m
                    664  474  m
                    668  450  m
                    670  430  m
                    672  408  m
                    676  382  m
                    678  358  m
                    678  329  m
                    678  302  m
                    678  274  m
                    678  245  m
                    678  221  m
                    678  199  m
                    678  184  m
                    678  168  m
                    678  152  m
                    678  152  m
                    678  172  m
                    678  198  m
                    680  231  m
                    680  265  m
                    680  302  m
                    682  334  m
                    682  369  m
                    682  396  m
                    682  422  m
                    682  446  m
                    682  462  m
                    680  478  m
                    678  466  m
                    678  452  m
                    677  438  m
                    677  420  m
                    677  399  m
                    677  381  m
                    677  362  m
                    677  347  m
                    677  340  m
                    677  336  e
                    675  336  m
                    663  336  m
                    647  336  m
                    628  336  m
                    600  336  m
                    568  336  m
                    524  332  m
                    474  328  m
                    412  320  m
                    352  314  m
                    308  306  m
                    270  300  m
                    246  296  m
                    228  293  m
                    228  297  m
                    236  308  m
                    243  318  m
                    250  327  m
                    255  335  m
                    257  346  m
                    261  355  m
                    267  364  m
                    272  366  m
                    278  371  m
                    288  375  m
                    303  378  m
                    315  381  m
                    321  384  m
                    324  377  m
                    322  369  m
                    321  375  m
                    322  384  m
                    325  399  m
                    327  418  m
                    329  445  m
                    331  478  m
                    337  503  m
                    343  537  m
                    349  570  m
                    354  600  m
                    358  621  m
                    362  637  m
                    363  649  m
                    363  659  m
                    363  669  m
                    362  682  m
                    362  693  m
                    362  704  m
                    361  714  m
                    360  716  m
                    359  705  m
                    359  692  m
                    359  673  m
                    361  647  m
                    361  618  m
                    363  584  m
                    365  553  m
                    365  522  m
                    367  491  m
                    367  458  m
                    367  420  m
                    367  386  m
                    367  349  m
                    365  311  m
                    363  287  m
                    361  263  m
                    359  246  m
                    357  230  m
                    354  218  m
                    352  208  m
                    352  197  m
                    350  188  m
                    349  177  m
                    348  168  m
                    348  157  m
                    349  157  m
                    350  166  m
                    350  176  m
                    350  187  m
                    350  197  m
                    350  207  m
                    352  215  m
                    349  223  m
                    346  234  m
                    343  244  m
                    340  245  m
                    337  238  m
                    340  237  m
                    334  237  b
                    334  237  e
                ",
            );

            inject_touches(
                "
                    337  351  b
                    337  351  e
                ",
            );

            assert_eq!(INDEX.load(Ordering::Relaxed), 666_665);

            Ok(())
        }
    }
}
