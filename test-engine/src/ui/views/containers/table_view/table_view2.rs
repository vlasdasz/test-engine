use refs::{Own, Weak};
use ui::{__ViewInternalTableData, Setup, View, ViewData, view_test};

use crate::{self as test_engine, ui::ScrollView};

#[view_test]
pub struct TableView2 {
    pub(super) data: Weak<dyn __ViewInternalTableData>,

    #[init]
    scroll: ScrollView,
}

impl Setup for TableView2 {
    fn setup(self: Weak<Self>) {
        self.scroll.place().back();
    }
}

impl TableView2 {
    fn cells(&self) -> &[Own<dyn View>] {
        self.scroll.content.scroll_content_subviews()
    }

    // fn get_free_or_create_cell

    fn get_cell_for_index(&self, index: usize) -> &dyn View {
        todo!()
    }
}
