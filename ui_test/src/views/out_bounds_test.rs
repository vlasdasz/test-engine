use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Anchor, IntView, Label, SubView, ViewData, ViewFrame, ViewSetup},
    App,
};

use crate::{view_tests::inject_touches, views::image_view::check_colors};

#[view]
struct OutBoundsView {
    test: SubView<Label>,
    x:    SubView<IntView>,
    y:    SubView<IntView>,
}

impl ViewSetup for OutBoundsView {
    fn setup(mut self: Weak<Self>) {
        self.test.set_text("AA").set_text_size(100).set_frame((200, 200, 200, 200));
        self.x.set_step(50);
        self.x
            .on_change(move |val| {
                self.test.set_x(200.0 + val);
            })
            .place()
            .size(60, 200)
            .center();

        self.y.set_step(50);
        self.y
            .on_change(move |val| {
                self.test.set_y(200.0 + val);
            })
            .place()
            .size(60, 200)
            .center_y()
            .anchor(Anchor::Left, self.x, 10);
    }
}

pub async fn test_out_bounds() -> Result<()> {
    App::init_test_view::<OutBoundsView>(600, 600).await;

    inject_touches(
        r#"
            296  368  b
            296  368  e
            298  370  b
            298  370  e
            297  370  b
            297  370  e
            298  370  b
            298  370  e
            298  370  b
            298  370  e
            298  371  b
            298  371  e
            298  372  b
            298  372  e
    "#,
    )
    .await;

    check_colors(
        r#"
               9  319 -   0   0   0
              21  319 - 255 255 255
              43  317 - 255 255 255
              63  318 -  25  51  76
              29  424 -  25  51  76
              34  378 - 255 255 255
              35  346 - 255 255 255
              37  301 - 255 255 255
              41  232 - 255 255 255
              34  186 -  25  51  76

    "#,
    )
    .await?;

    inject_touches(
        r#"
            363  376  b
            363  376  e
            362  376  b
            362  376  e
            362  375  b
            362  375  e
            363  375  b
            363  375  e
            364  375  b
            364  375  e
            364  375  b
            364  375  e
        "#,
    )
    .await;

    check_colors(
        r#"
              21   39 - 255 255 255
               6   15 -   0   0   0
              69   36 -  25  51  76
              40  119 -  25  51  76
              27   66 - 255 255 255
        "#,
    )
    .await?;

    inject_touches(
        r#"
            311  302  b
            311  301  e
            310  301  b
            310  301  e
            311  301  b
            311  301  e
            312  300  b
            312  300  e
            312  300  b
            312  300  e
            312  300  b
            312  300  e
            312  300  b
            312  300  e
            312  300  b
            312  300  e
            311  300  b
            311  300  e
            311  300  b
            311  300  e
            311  300  b
            311  300  e
            311  300  b
            311  300  e
        "#,
    )
    .await;

    check_colors(
        r#"
             427   40 -  25  51  76
             513   27 - 255 255 255
             534   17 - 255 255 255
             554   12 - 255 255 255
             570   10 - 255 255 255
             577   50 - 255 255 255
             573   76 - 255 255 255
             576  116 -  25  51  76
             574  132 -  25  51  76
             419  123 -  25  51  76
        "#,
    )
    .await?;

    inject_touches(
        r#"
            372  296  b
            372  296  e
            372  295  b
            371  295  e
            370  296  b
            370  295  e
            371  295  b
            371  295  e
            370  296  b
            370  296  e
            370  296  b
            370  296  e
            370  296  b
            370  296  e
            370  296  b
            370  296  e
            370  296  b
            370  296  e
            370  296  b
            370  296  e
            370  296  b
            370  296  e
            370  296  b
            370  296  e
        "#,
    )
    .await;

    check_colors(
        r#"
             425  565 -  25  51  76
             478  573 - 255 255 255
             513  577 -  15  15  15
             527  575 -   0   0   0
             561  575 - 255 255 255
             587  535 - 255 255 255
             563  477 -  25  51  76
             411  484 -  25  51  76
        "#,
    )
    .await?;

    inject_touches(
        r#"
            301  374  b
            301  374  e
            302  374  b
            302  374  e
            304  374  b
            304  374  e
            304  374  b
            304  374  e
            304  374  b
            304  374  e
            304  374  b
            304  374  e
            304  374  b
            304  374  e
            304  374  b
            304  374  e
            304  374  b
            304  374  e
            304  374  b
            304  374  e
            304  374  b
            304  374  e
            304  374  b
            304  374  e
            303  301  b
            303  301  e
        "#,
    )
    .await;

    check_colors(
        r#"
              23  574 -   0   0   0
              38  573 -   0   0   0
              96  566 - 255 255 255
             124  558 -  25  51  76
              96  466 -  25  51  76
              46  481 -  25  51  76
              33  518 - 255 255 255
        "#,
    )
    .await?;

    debug!("Out bounds test: OK");

    Ok(())
}
