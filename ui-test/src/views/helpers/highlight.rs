use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{BLUE, GREEN, HighlightView, Setup, UI, view},
    ui_test::check_colors,
};

#[view]
struct HighLightTestView {
    #[init]
    highlight: HighlightView,
}

impl Setup for HighLightTestView {
    fn setup(mut self: Weak<Self>) {
        self.highlight.set((200, 200), GREEN, BLUE);
    }
}

pub async fn test_highlight() -> Result<()> {
    UI::init_test_view::<HighLightTestView>().await;

    check_colors(
        r#"
              76  258 -  89 124 149
             115  258 -  89 124 149
             118  258 -  89 124 149
             163  258 -   0 255   0
             168  258 -   0 255   0
             178  256 -   0 255   0
             186  256 -   0 255   0
             192  256 -   0 255   0
             228  256 -   0 255   0
             228  256 -   0 255   0
             228  256 -   0 255   0
             241  252 -   0   0 231
             241  252 -   0   0 231
             245  233 -   0   0 231
             236  223 -   0   0 231
             281  182 -  89 124 149
             292  182 -  89 124 149
             249  182 -   0   0 231
             249  182 -   0   0 231
             240  182 -   0   0 231
             199  182 -  89 124 149
             199  182 -  89 124 149
             128  191 -   0 255   0
             169  191 -  89 124 149
             169  187 -  89 124 149
             169  165 -  89 124 149
             196  163 -   0 255   0
             200  160 -   0 255   0
             211  150 -   0 255   0
             222  150 -   0 255   0
             250  150 -   0   0 231
             250  150 -   0   0 231
             250   92 -  89 124 149
             250   86 -  89 124 149
             207   86 -  89 124 149
             196   86 -  89 124 149
        "#,
    )
    .await?;

    Ok(())
}
