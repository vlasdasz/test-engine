use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{Alert, Setup, UI, view},
    ui_test::{check_colors, inject_touches},
};

#[view]
struct AlertTestView {}

impl Setup for AlertTestView {
    fn setup(self: Weak<Self>) {}
}

pub async fn test_alert() -> Result<()> {
    UI::init_test_view::<AlertTestView>().await;

    from_main(|| {
        Alert::show("Forogorn\nSopokok\nFergel");
    })
    .await;

    check_colors(
        r#"
             280  327 - 255 255 255
             290  294 - 255 255 255
             296  272 - 255 255 255
             307  244 - 184 184 184
             347  228 - 255 255 255
             364  269 - 255 255 255
             337  282 -   1   1   1
             295  291 - 255 255 255
             268  312 - 255 255 255
             145  312 -  25  51  76
             218  174 -  25  51  76
             477  253 -  25  51  76
             387  315 - 255 255 255
             277  378 - 255 255 255
             240  399 - 255 255 255
             240  447 -  25  51  76
             297  270 - 255 255 255
        "#,
    )
    .await?;

    inject_touches(
        "
            338  373  b
            338  373  e
        ",
    )
    .await;

    check_colors(
        r#"
        134  271 -  25  51  76
        189  271 -  25  51  76
        290  271 -  25  51  76
        365  271 -  25  51  76
        472  271 -  25  51  76
        544  271 -  25  51  76
   "#,
    )
    .await?;

    debug!("Alert test: OK");

    Ok(())
}
