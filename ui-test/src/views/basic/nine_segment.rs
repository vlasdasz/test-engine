use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{NineSegmentImageView, Setup, UI, ViewFrame, view},
    ui_test::{check_colors, record_ui_test},
};

#[view]
struct NineSegment {
    #[init]
    segment: NineSegmentImageView,
}

impl Setup for NineSegment {
    fn setup(mut self: Weak<Self>) {
        self.segment.set_frame((50, 50, 200, 200));

        self.segment.set_image("button");
    }
}

pub async fn test_nine_segment() -> Result<()> {
    let _view = UI::init_test_view::<NineSegment>().await;

    check_colors(
        r#"
              36   61 -  89 124 149
              64   69 -   0  61 177
              64   71 -   0  56 163
             124  120 -   6  19  64
             150  125 -   5  19  66
             174  131 -   3  18  65
             179  132 -   5  19  65
             188  134 -   4  19  65
             197  148 -   4  18  63
             217  154 -   4  18  62
             217  154 -   4  18  62
             222  154 -   3  17  62
             222  154 -   3  17  62
             251  187 -  89 124 149
             292  189 -  89 124 149
              42  249 -  89 124 149
              42  249 -  89 124 149
              59  223 -   0  58 168
              61  223 -   0  55 160
              65  223 -   3   6  31
              69  223 -   3  18  62
              79  215 -   3  18  63
              83  208 -   3  18  63
              87  197 -   4  15  62
              96  187 -   5  19  63
             104  185 -   4  18  64
             122  168 -   4  18  64
             143  131 -   5  19  66
             143  128 -   5  20  66
             157  109 -   4  19  66
             157  108 -   3  17  66
             238   78 -   0  59 168
             238   78 -   0  59 168
             301  210 -  89 124 149
             299  201 -  89 124 149
             227  179 -   4  17  63
             225  178 -   4  16  61
             187  166 -   4  19  64
             117  148 -   5  19  66
              53  137 -   0  54 153
              23  136 -  89 124 149
              45   48 -  89 124 149
             122  111 -   4  18  65
        "#,
    )
    .await?;

    record_ui_test().await;

    Ok(())
}
