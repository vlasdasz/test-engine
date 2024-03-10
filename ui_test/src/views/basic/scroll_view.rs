use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, Color, ScrollView, SubView, ViewData, ViewSetup},
    App,
};

use crate::utils::{
    helpers::{add_corners, check_colors},
    inject_scroll, inject_touches,
};

#[view]
struct ScrollViewTest {
    scroll: SubView<ScrollView>,
}

impl ViewSetup for ScrollViewTest {
    fn setup(mut self: Weak<Self>) {
        self.scroll.content_size = (600, 600).into();
        self.scroll.place().back();
        add_corners(self.scroll, Color::TURQUOISE);
    }
}

pub async fn test_scroll_view() -> Result<()> {
    let mut view = App::init_test_view::<ScrollViewTest>(600, 600).await;

    check_colors(
        r#"
              53  554 -   0 255 255
             168  556 -  25  51  76
             340  553 -  25  51  76
             480  553 -  25  51  76
             532  547 -   0 255 255
             561  469 -  25  51  76
             575  372 -  25  51  76
             566  220 -  25  51  76
             561  134 -  25  51  76
             561   60 -   0 255 255
             440   35 -  25  51  76
             293   41 -  25  51  76
             164   45 -  25  51  76
              93   46 -   0 255 255
              60   71 -   0 255 255
             135  403 -  25  51  76
             382  305 -  25  51  76
        "#,
    )
    .await?;

    assert_eq!(view.scroll.content_offset.y, 0.0);

    inject_scroll(5).await;
    assert_eq!(view.scroll.content_offset.y, 5.0);

    inject_scroll(20).await;
    assert_eq!(view.scroll.content_offset.y, 25.0);

    inject_scroll(30).await;
    assert_eq!(view.scroll.content_offset.y, 55.0);

    check_colors(
        r#"
              51   35 -  25  51  76
              49   62 -   0 255 255
              49   91 -   0 255 255
              50  142 -   0 255 255
              48  171 -  25  51  76
             569  185 -  25  51  76
             571  147 -   0 255 255
             571  112 -   0 255 255
             570   48 -  25  51  76
             555   25 -  25  51  76
             481   95 -  25  51  76
             144  116 -  25  51  76
             107  577 -  25  51  76
              76  578 -   0 255 255
              61  567 -   0 255 255
              46  530 -  25  51  76
             457  573 -  25  51  76
             517  582 -   0 255 255
             542  575 -   0 255 255
             558  523 -  25  51  76
        "#,
    )
    .await?;

    inject_scroll(-55).await;
    assert_eq!(view.scroll.content_offset.y, 0.0);

    check_colors(
        r#"
             506  554 -   0 255 255
             477  553 -  25  51  76
             556  488 -  25  51  76
             540  512 -   0 255 255
             120  547 -  25  51  76
              85  544 -   0 255 255
              62  513 -   0 255 255
              60  485 -  25  51  76
              62  110 -  25  51  76
              67   89 -   0 255 255
              85   50 -   0 255 255
             111   31 -  25  51  76
             485   30 -  25  51  76
             511   35 -   0 255 255
             567   68 -   0 255 255
             565  119 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.scroll.content_size = (400, 400).into();
    })
    .await;

    check_colors(
        r#"
              75  409 -  25  51  76
              77  381 -   0 255 255
             109  379 -  25  51  76
             118  309 -  25  51  76
              87  308 -   0 255 255
              70  277 -  25  51  76
              80  108 -  25  51  76
              87   71 -   0 255 255
             116   73 -  25  51  76
             275   77 -  25  51  76
             321   77 -   0 255 255
             337  114 -  25  51  76
             384  113 -  25  51  76
             383   84 -   0 255 255
             409   86 -  25  51  76
             413  324 -  25  51  76
             353  312 -   0 255 255
             265  315 -  25  51  76
             338  274 -  25  51  76
             316  383 -   0 255 255
             322  418 -  25  51  76
             392  414 -  25  51  76
             380  362 -   0 255 255
             432  373 -  25  51  76
              30  417 -  25  51  76
              31  377 -   0 255 255
              44  558 -  25  51  76
             272  550 -  25  51  76
             542  555 -  25  51  76
             565  425 -  25  51  76
             545  131 -  25  51  76
             550   40 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.scroll.content_size = (600, 1200).into();
    })
    .await;

    inject_touches(
        r#"
            583  25   b
            503  580  m
            503  580  e
        "#,
    )
    .await;

    check_colors(
        r#"
             540  479 -  25  51  76
             537  529 -   0 255 255
             510  541 -   0 255 255
             488  541 -  25  51  76
             115  561 -  25  51  76
              72  559 -   0 255 255
              72  510 -   0 255 255
              72  476 -  25  51  76
              31   27 -  25  51  76
              55   27 -  25  51  76
             533   46 -  25  51  76
             582   46 -  25  51  76
        "#,
    )
    .await?;

    debug!("Scroll view test: OK");

    Ok(())
}
