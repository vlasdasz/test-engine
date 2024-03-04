use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{view, Color, ScrollView, SubView, ViewData, ViewSetup},
    App,
};

use crate::{
    view_tests::{inject_scroll, record_ui_test},
    views::{helpers::add_corners, image_view::check_colors},
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
    let view = App::init_test_view::<ScrollViewTest>(600, 600).await;

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

    inject_scroll(5).await;
    inject_scroll(20).await;
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

    record_ui_test().await?;

    Ok(())
}
