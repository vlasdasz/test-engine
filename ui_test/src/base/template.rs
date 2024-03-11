use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, ViewSetup},
    App,
};

#[view]
struct TemplateTestView {}

impl ViewSetup for TemplateTestView {
    fn setup(self: Weak<Self>) {}
}

pub async fn test_template() -> Result<()> {
    App::init_test_view::<TemplateTestView>().await;

    debug!("Template test: OK");

    Ok(())
}
