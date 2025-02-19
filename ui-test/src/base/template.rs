use anyhow::Result;
use test_engine::ui::{UI, view};

#[view]
struct TemplateView<Value: 'static> {
    value: Value,
}

pub async fn test_template() -> Result<()> {
    let view = UI::init_test_view::<TemplateView<i32>>().await;

    assert_eq!(view.value, 0);

    Ok(())
}
