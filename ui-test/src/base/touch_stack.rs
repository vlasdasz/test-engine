use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    ui::{view, Alert, Button, Sub, TouchStack, ViewSubviews, ViewTouch, UI},
    ui_test::inject_touches,
    wait_for_next_frame,
};

#[view]
struct TouchStackTestView {
    #[text = a]
    button:  Sub<Button>,
    #[text = b]
    button2: Sub<Button>,
}

pub async fn test_touch_stack() -> Result<()> {
    let mut view = UI::init_test_view::<TouchStackTestView>().await;

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    let mut button = from_main(move || view.add_view::<Button>()).await;

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    button.on_tap(|| {});

    assert_eq!(
        TouchStack::dump(),
        vec![vec!["Layer: Root view".to_string(), button.view_label.clone()]],
    );

    from_main(move || button.remove_from_superview()).await;

    wait_for_next_frame().await;

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    view.button.on_tap(|| {});

    assert_eq!(
        TouchStack::dump(),
        vec![vec![
            "Layer: Root view".to_string(),
            view.button.view_label.clone()
        ]],
    );

    view.button2.on_tap(|| {});

    assert_eq!(
        TouchStack::dump(),
        vec![vec![
            "Layer: Root view".to_string(),
            view.button.view_label.clone(),
            view.button2.view_label.clone(),
        ]],
    );

    view.button.disable_touch();
    view.button2.disable_touch();

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    Alert::show("Hello");

    wait_for_next_frame().await;

    assert_eq!(
        TouchStack::dump(),
        vec![
            vec!["Layer: Root view".to_string()],
            vec!["Layer: Alert".to_string(), "Alert.ok_button: Button".to_string()],
        ],
    );

    inject_touches(
        r#"
            320  383  b
            320  383  e
    "#,
    )
    .await;

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    debug!("Touch stack test: OK");

    Ok(())
}
