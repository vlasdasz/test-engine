use anyhow::Result;
use test_engine::{
    inspect::views::AnchorView,
    refs::Weak,
    ui::{Anchor, Setup, UI, ViewFrame, view},
    ui_test::{check_colors, record_ui_test},
};

#[view]
struct AnchorViewTest {
    #[init]
    anchor_view: AnchorView,
}

impl Setup for AnchorViewTest {
    fn setup(mut self: Weak<Self>) {
        self.anchor_view.set_frame((50, 50, 200, 200));
        self.anchor_view.set_anchor(Anchor::Top);
    }
}

pub(crate) async fn test_anchor_view() -> Result<()> {
    let _view = UI::init_test_view::<AnchorViewTest>();

    check_colors(
        r#"
              40   77 -  89 124 149
              98   79 - 255 255 255
              99   59 - 255   0   0
             105   29 -  89 124 149
             222   32 -  89 124 149
             266   48 -  89 124 149
             228   59 - 255   0   0
             234   86 - 255 255 255
             244  162 - 255 255 255
             265  162 -  89 124 149
             105  236 - 255 255 255
              64  194 - 255 255 255
        "#,
    )?;

    record_ui_test();

    Ok(())
}
