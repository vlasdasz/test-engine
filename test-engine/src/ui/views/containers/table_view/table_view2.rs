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
    use anyhow::Result;
    use refs::{Own, Weak};
    use ui::{Label, Setup, UIManager, View, ViewData, ViewTest, view_test};

    use crate::{
        self as test_engine,
        ui::{CellRegistry, TableData, TableView2},
    };

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

            cell
        }
    }

    impl ViewTest for TableView2Test {
        fn perform_test(view: Weak<Self>) -> Result<()> {
            UIManager::enable_debug_frames();
            crate::ui_test::record_ui_test();
            Ok(())
        }
    }
}
