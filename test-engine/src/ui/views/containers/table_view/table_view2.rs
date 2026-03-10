use std::{any::type_name, ops::DerefMut};

use anyhow::Result;
use netrun::Function;
use refs::{Own, Weak};
use ui::{Setup, View, ViewData, ViewFrame, ViewSubviews, ViewTest, view_test};

use crate::{
    self as test_engine,
    ui::{CellRegistry, ScrollView, TableData, struct_name},
};

#[view_test]
pub struct TableView2 {
    pub(super) data: Weak<dyn TableData>,

    #[educe(Default = 1)]
    pub(super) columns: usize,

    pub(super) registry: CellRegistry,

    #[init]
    pub(super) scroll: ScrollView,
}

impl Setup for TableView2 {
    fn setup(mut self: Weak<Self>) {
        self.scroll.place().back();

        self.scroll.on_scroll.sub(move || {
            self.layout_cells();
        });

        self.size_changed().sub(move || {
            self.layout_cells();
        });
    }
}

impl TableView2 {
    pub fn set_data_source(mut self: Weak<Self>, data: Weak<dyn TableData>) -> Weak<Self> {
        self.data = data;
        self
    }

    pub fn register_cell<T: View>(
        mut self: Weak<Self>,
        mut create: impl FnMut() -> Own<dyn View> + Send + 'static,
    ) {
        self.registry
            .constructors
            .insert(struct_name::<T>(), Function::new(move |()| create()));
    }

    pub fn reload_data(&mut self) {
        self.layout_cells();
    }
}

impl TableView2 {
    fn layout_cells(&mut self) {
        if self.height() <= 0.0 {
            return;
        }

        assert!(
            self.data.is_ok(),
            "TableView data source is not set. Use set_data_source method."
        );

        let number_of_cells = self.data.number_of_cells();

        if number_of_cells == 0 {
            return;
        }

        if self.data.variable_height() {
            assert_eq!(
                self.columns, 1,
                "Variable height supported only for tables with 1 column"
            );
            // layout_variable_sized_cells(self, number_of_cells);
        } else {
            match self.columns {
                1 => self.layout_single_column_cells_2(number_of_cells),
                // 2 => layout_two_column_cells(self, number_of_cells),
                _ => unimplemented!("More than TableView 2 columns is not supported yet"),
            }
        }
    }
}

impl ViewTest for TableView2 {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        crate::ui_test::record_ui_test();

        Ok(())
    }
}

mod test {
    use std::ops::Deref;

    use anyhow::Result;
    use gm::color::Color;
    use parking_lot::Mutex;
    use refs::{Own, Weak};
    use ui::{AfterSetup, Label, Setup, View, ViewData, ViewTest, view_test};

    use crate::{
        self as test_engine,
        ui::{CellRegistry, TableData, TableView2},
        ui_test::{inject_scroll, inject_touches},
    };

    static TEST_DATA: Mutex<String> = Mutex::new(String::new());

    #[view_test]
    struct TableView2Test {
        #[init]
        table: TableView2,
    }

    impl Setup for TableView2Test {
        fn setup(mut self: Weak<Self>) {
            self.table.place().back();
            self.table.set_data_source(self);
            self.table.register_cell::<Label>(|| Label::new());
            self.table.reload_data();
        }
    }

    impl TableData for TableView2Test {
        fn cell_height(&self, _: usize) -> f32 {
            100.0
        }

        fn number_of_cells(&self) -> usize {
            10000
        }

        fn setup_cell2(&self, index: usize, registry: &mut CellRegistry) -> Own<dyn View> {
            let cell = registry.get_cell::<Label>();
            cell.set_text(index);
            cell.set_border_width(index % 20);
            cell.set_color(Color::ALL[index % Color::ALL.len()]);
            cell.set_border_color(Color::ALL[(index + 1) % Color::ALL.len()]);
            cell.set_corner_radius(index % 40);

            cell
        }

        fn cell_selected(&mut self, index: usize) {
            *TEST_DATA.lock() += &format!("|{index}|");
        }
    }

    impl ViewTest for TableView2Test {
        fn perform_test(_view: Weak<Self>) -> Result<()> {
            inject_touches(
                "
                    395  35   b
                    394  35   e
                    357  160  b
                    357  159  e
                    349  258  b
                    349  258  e
                    351  366  b
                    351  366  e
                    353  455  b
                    353  455  e
                    350  528  b
                    350  528  e
                ",
            );

            assert_eq!(TEST_DATA.lock().deref(), "|0||1||2||3||4||5|");

            TEST_DATA.lock().clear();

            for _ in 0..200 {
                inject_scroll(-20);
            }

            inject_scroll(-1000);

            inject_touches(
                "
                359  58   b
                359  58   e
                334  159  b
                334  159  e
                349  239  b
                349  239  e
                354  346  b
                353  345  e
                354  436  b
                353  435  e
                353  536  b
                353  536  e

            ",
            );

            assert_eq!(TEST_DATA.lock().deref(), "|50||51||52||53||54||55|");

            crate::ui_test::record_ui_test();
            Ok(())
        }
    }
}
