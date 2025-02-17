use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{Setup, UI, view},
    ui_test::record_ui_test,
};

#[view]
struct ColorsTestView {}

impl Setup for ColorsTestView {
    fn setup(self: Weak<Self>) {}
}

pub async fn test_colors() -> Result<()> {
    UI::init_test_view::<ColorsTestView>().await;

    record_ui_test().await;

    debug!("Colors test: OK");
    Ok(())
}
