use anyhow::Result;
use log::debug;
use test_engine::Screen;
use ui::{refs::Weak, view, SubView, TouchStack, ViewSetup};
use ui_views::Button;

use crate::view_tests::{
    assert_eq, inject_touches, record_touches,
    state::{append_state, get_str_state},
};

#[view]
struct TouchStackTestView {
    #[text = a]
    button:  SubView<Button>,
    #[text = b]
    button2: SubView<Button>,
}

impl ViewSetup for TouchStackTestView {
    fn setup(self: Weak<Self>) {
        self.button.place.back();
        self.button2.place.back();
    }
}

pub async fn test_touch_stack() -> Result<()> {
    Screen::set_test_view::<TouchStackTestView>(600, 600).await;

    let this = TouchStackTestView::instance();

    assert_eq(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]])?;

    this.button.on_tap(|| append_state("1"));

    assert_eq(
        TouchStack::dump(),
        vec![vec![
            "Layer: Root view".to_string(),
            "View: ButtonTouchStackTestView.button".to_string(),
        ]],
    )?;

    inject_touches(
        r#"
         5  5 b
        10 10 e
     "#,
    )
    .await;

    assert_eq(get_str_state(), "1")?;

    this.button2.on_tap(|| {
        append_state("2");
    });

    assert_eq(
        TouchStack::dump(),
        vec![vec![
            "Layer: Root view".to_string(),
            "View: ButtonTouchStackTestView.button2".to_string(),
            "View: ButtonTouchStackTestView.button".to_string(),
        ]],
    )?;

    inject_touches(
        r#"
         5  5 b
        10 10 e
     "#,
    )
    .await;

    assert_eq(get_str_state(), "12")?;

    record_touches().await;

    debug!("Touch stack test: OK");

    Ok(())
}
