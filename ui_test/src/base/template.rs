use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, ViewSetup},
};

#[view]
struct TemplateTestView {}

impl ViewSetup for TemplateTestView {
    fn setup(self: Weak<Self>) {
        todo!()
    }
}

pub async fn test_template() -> Result<()> {
    debug!("Template test: OK");

    Ok(())
}
