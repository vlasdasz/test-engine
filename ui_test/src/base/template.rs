use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, ViewSetup, UI},
};

#[view]
struct TemplateTestView {}

impl ViewSetup for TemplateTestView {
    fn setup(self: Weak<Self>) {}
}

pub async fn test_template() -> Result<()> {
    UI::init_test_view::<TemplateTestView>().await;

    debug!("Template test: OK");

    Ok(())
}
