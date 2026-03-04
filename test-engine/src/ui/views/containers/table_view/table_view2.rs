use ui::{ScrollView, view_test};

use crate::{self as test_engine};

#[view_test]
pub struct TableView2 {
    #[init]
    scroll: ScrollView,
}
