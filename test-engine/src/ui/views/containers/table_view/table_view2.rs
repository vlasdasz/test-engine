use std::{any::type_name, collections::HashMap};

use anyhow::Result;
use netrun::Function;
use refs::{Own, Weak};
use ui::{__ViewInternalTableData, Setup, View, ViewData, ViewTest, view_test};

use crate::{
    self as test_engine,
    ui::{ScrollView, cell_registry::CellRegistry},
};

#[view_test]
pub struct TableView2 {
    pub(super) data: Weak<dyn __ViewInternalTableData>,

    registry: CellRegistry,

    #[init]
    scroll: ScrollView,
}

impl Setup for TableView2 {
    fn setup(self: Weak<Self>) {
        self.scroll.place().back();
    }
}

impl TableView2 {
    pub fn register_cell<T: View>(
        mut self: Weak<Self>,
        mut create: impl FnMut() -> Own<dyn View> + Send + 'static,
    ) {
        self.registry
            .constructors
            .insert(type_name::<T>(), Function::new(move |()| create()));
    }

    pub fn get_cell<T: View + 'static>(&mut self) -> Own<T> {
        let cell = self.registry.cell_for_ident(type_name::<T>());

        cell.downcast::<T>()
    }
}

impl TableView2 {
    fn cells(&self) -> &[Own<dyn View>] {
        self.scroll.content.scroll_content_subviews()
    }

    // fn get_free_or_create_cell

    fn _get_cell_for_index(&self, _index: usize) -> &dyn View {
        todo!()
    }
}

impl ViewTest for TableView2 {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        crate::ui_test::record_ui_test();

        Ok(())
    }
}
