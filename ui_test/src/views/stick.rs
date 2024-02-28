use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{view, StickView, SubView, ViewData, ViewSetup},
    App,
};

use crate::view_tests::record_ui_test;

#[view]
struct StickTestView {
    stick: SubView<StickView>,
}

impl ViewSetup for StickTestView {
    fn setup(self: Weak<Self>) {
        self.stick.place().size(200, 200).tl(100);
    }
}

pub async fn test_stick() -> Result<()> {
    App::init_test_view::<StickTestView>(600, 600).await;

    record_ui_test().await?;

    Ok(())
}
