use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{view, ScrollView, SubView, TouchStack, ViewData, ViewSetup},
    App,
};

use crate::view_tests::record_ui_test;

#[view]
struct ScrollViewTest {
    scroll: SubView<ScrollView>,
}

impl ViewSetup for ScrollViewTest {
    fn setup(self: Weak<Self>) {
        self.scroll.place().back();
    }
}

pub async fn test_scroll_view() -> Result<()> {
    App::init_test_view::<ScrollViewTest>(600, 600).await;

    dbg!(TouchStack::dump());

    record_ui_test().await?;

    Ok(())
}
