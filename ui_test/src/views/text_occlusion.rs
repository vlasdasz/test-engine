use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{view, Color, Label, SubView, ViewData, ViewSetup},
    App,
};

use crate::view_tests::record_touches;

#[view]
pub struct TextOccclusionTestView {
    label_below: SubView<Label>,
    label_above: SubView<Label>,
}

impl ViewSetup for TextOccclusionTestView {
    fn setup(mut self: Weak<Self>) {
        self.label_below
            .set_text_size(100)
            .set_text("OOOO")
            .place()
            .center()
            .size(400, 400);

        self.label_above
            .set_text_size(100)
            .set_color(Color::LIGHT_GRAY)
            .set_text("AAAA")
            .place()
            .right_half();
    }
}

pub async fn test_text_occlusion() -> Result<()> {
    App::set_test_view::<TextOccclusionTestView>(600, 600).await;

    record_touches().await;

    Ok(())
}
