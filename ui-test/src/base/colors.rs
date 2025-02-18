use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{ImageView, Setup, UI, ViewData, view},
    ui_test::record_ui_test,
};

#[view]
struct ColorsTestView {
    #[init]
    image: ImageView,
}

impl Setup for ColorsTestView {
    fn setup(mut self: Weak<Self>) {
        self.image.place().tl(20).size(400, 520);
        self.image.set_image("colors.png");
    }
}

pub async fn test_colors() -> Result<()> {
    UI::init_test_view::<ColorsTestView>().await;

    record_ui_test().await;

    debug!("Colors test: OK");
    Ok(())
}
