use anyhow::Result;
use log::debug;
use test_engine::{
    reflected,
    reflected::Reflected,
    refs::Weak,
    ui::{view, Color, HighlightView, ViewSetup, UI},
    ui_test::check_colors,
};

#[derive(Default, Debug, Reflected)]
struct Data {
    float_field:   f32,
    integer_field: u32,
    boolean:       bool,
    string:        String,
}

#[view]
struct HighLightTestView {
    #[init]
    highlight: HighlightView,
}

impl ViewSetup for HighLightTestView {
    fn setup(mut self: Weak<Self>) {
        self.highlight.set((200, 200), Color::GREEN, Color::BLUE);
    }
}

pub async fn test_highlight() -> Result<()> {
    UI::init_test_view::<HighLightTestView>().await;

    check_colors(
        r#"
              96  213 -  25  51  76
              97  213 -  25  51  76
             106  212 -  25  51  76
             127  209 -   0 255   0
             133  207 -   0 255   0
             149  204 -   0 255   0
             168  201 -  25  51  76
             177  201 -  25  51  76
             195  198 -  25  51  76
             208  197 -  25  51  76
             231  197 -  25  51  76
             241  197 -   0   0 203
             261  197 -   0   0 203
             298  200 -  25  51  76
             305  201 -  25  51  76
             328  201 -  25  51  76
             204   79 -  25  51  76
             202   92 -  25  51  76
             199  120 -  25  51  76
             200  131 -   0 255   0
             207  158 -   0 255   0
             210  178 -  25  51  76
             210  199 -  25  51  76
             211  217 -  25  51  76
             208  247 -   0 255   0
             212  281 -  25  51  76
             212  286 -  25  51  76
             212  299 -  25  51  76
             218  358 -  25  51  76
        "#,
    )
    .await?;

    debug!("HighLight view: OK");

    Ok(())
}
