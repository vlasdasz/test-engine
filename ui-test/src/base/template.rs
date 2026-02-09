use anyhow::Result;
use test_engine::{ui::view, ui_test::UITest};

#[view]
struct TemplateView<Value: 'static> {
    value: Value,
}

pub async fn test_template() -> Result<()> {
    let view = UITest::start::<TemplateView<i32>>();

    assert_eq!(view.value, 0);

    Ok(())
}
