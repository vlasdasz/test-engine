use anyhow::Result;
use instant::Instant;
use test_engine::{
    dispatch::from_main,
    ui::{
        Container, NavigationView, PRESENT_ANIMATION_DURATION, RED, Setup, TouchStack, UIDrawer,
        ViewController, ViewData, view,
    },
    ui_test::helpers::check_colors,
};

#[view]
struct PresentTestView {}

pub async fn test_navigation_view() -> Result<()> {
    let present = PresentTestView::new();

    let view = present.weak();

    UIDrawer::set_test_view(
        NavigationView::with_view(present),
        600,
        600,
        true,
        "Present".to_string(),
    );

    check_colors(
        r#"
              50   69 -  89 124 149
              42   40 -  89 124 149
              30   25 -  89 124 149
              69   34 -  89 124 149
             118   52 -  89 124 149
             184   59 -  89 124 149
             293   88 -  89 124 149
             333  138 -  89 124 149
             258  219 -  89 124 149
             173  294 -  89 124 149
             333  385 -  89 124 149
        "#,
    )?;

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    let now = Instant::now();

    let presented = from_main(move || {
        let presented = Container::new();
        presented.set_color(RED);

        view.present(presented)
    });

    presented.recv()?;

    let duration_error = now.elapsed().as_secs_f32() - PRESENT_ANIMATION_DURATION;
    let allowed_error = 0.032;

    check_colors(
        r#"
              90  169 - 255 255 255
             103  137 - 255 255 255
             331   86 - 255 255 255
             439  145 - 255 255 255
             470  253 - 255 255 255
             254  310 - 255 255 255
             168  363 - 255 255 255
             258  461 - 255 255 255
             409  465 - 255 255 255
             392  363 - 255 255 255
        "#,
    )?;

    assert!(
        duration_error < allowed_error,
        "Duration error is: {duration_error}. Allowed: {allowed_error}"
    );

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    check_colors(
        r#"
             176  243 - 255 255 255
             175  158 - 255 255 255
             308   80 - 255 255 255
             461  147 - 255 255 255
             388  350 - 255 255 255
             239  511 - 255 255 255
             202  532 - 255 255 255
        "#,
    )?;

    Ok(())
}
