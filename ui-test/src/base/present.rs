use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    ui::{
        view, Color, Container, NavigationView, Setup, TouchStack, ViewController, ViewData,
        PRESENT_ANIMATION_DURATION, UI,
    },
    ui_test::helpers::check_colors,
};
use tokio::time::Instant;

#[view]
struct PresentTestView {}

pub async fn test_present() -> Result<()> {
    let present = PresentTestView::new();

    let view = present.weak();

    UI::set_test_view(NavigationView::with_view(present), 600, 600).await;

    check_colors(
        r"
              32   28 -  25  51  76
             306  347 -  25  51  76
             547  566 -  25  51  76
        ",
    )
    .await?;

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    let now = Instant::now();

    let presented = from_main(move || {
        let mut presented = Container::new();
        presented.set_color(Color::RED);

        view.present(presented)
    })
    .await;

    check_colors(
        r"
              32   28 -  25  51  76
             306  347 -  25  51  76
             547  566 -  25  51  76
        ",
    )
    .await?;

    presented.await?;

    let duration_error = now.elapsed().as_secs_f32() - PRESENT_ANIMATION_DURATION;
    let allowed_error = 0.025;

    assert!(
        duration_error < allowed_error,
        "Duration error is: {duration_error}. Allowed: {allowed_error}"
    );

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    check_colors(
        r"
              53   22 - 255 255 255
             222  255 - 255 255 255
             490  551 - 255 255 255
        ",
    )
    .await?;

    debug!("Present test: OK");

    Ok(())
}
