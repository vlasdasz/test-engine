use refs::Weak;
use ui::{Setup, ViewData, view_test};

use crate::{self as test_engine, ui::ScrollView};

#[view_test]
pub struct TableView2 {
    #[init]
    scroll: ScrollView,
}

impl Setup for TableView2 {
    fn setup(self: Weak<Self>) {
        self.scroll.place().back();
    }
}
