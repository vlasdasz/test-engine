use anyhow::Result;
use test_engine::ui::{UIDrawer, view};

#[view]
struct TemplateView<Value: 'static> {
    value: Value,
}

pub async fn test_template() -> Result<()> {
    let view = UIDrawer::init_test_view::<TemplateView<i32>>();

    assert_eq!(view.value, 0);

    Ok(())
}
