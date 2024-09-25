use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Point, PositionView, Setup, UI},
    ui_test::inject_touches,
};

#[view]
struct PositionViewTest {
    #[init]
    pos: PositionView,
}

impl Setup for PositionViewTest {
    fn setup(self: Weak<Self>) {}
}

pub async fn test_position_view() -> Result<()> {
    let view = UI::init_test_view::<PositionViewTest>().await;

    inject_touches(
        "
            174  35   b
            402  369  m
            402  369  e
        ",
    )
    .await;

    assert_eq!(view.pos.position, Point::new(228.0, 334.0));

    inject_touches(
        "
            350  374  b
            460  162  m
            460  162  e

        ",
    )
    .await;

    assert_eq!(view.pos.position, Point::new(338.0, 122.0));

    inject_touches(
        "
            512  146  b
            144  540  m
            144  540  e
        ",
    )
    .await;

    assert_eq!(view.pos.position, Point::new(-30.0, 516.0));

    debug!("Position view: OK");

    Ok(())
}
