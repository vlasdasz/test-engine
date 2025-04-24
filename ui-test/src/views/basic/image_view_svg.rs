use anyhow::Result;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{Setup, UI, UIManager, ViewData, ViewTouch, WHITE, view},
    ui_test::{helpers::check_colors, record_ui_test},
};

#[view]
struct ImageViewSVG {
    #[init]
    image_view: test_engine::ui::ImageView,
}

impl Setup for ImageViewSVG {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();

        self.image_view.place().back(); //.tl(5).size(400, 400);
        self.image_view.set_image("bin.svg");
    }
}

pub async fn test_image_view_svg() -> Result<()> {
    from_main(|| {
        UIManager::set_clear_color(WHITE);
    })
    .await;

    let _view = UI::init_test_view::<ImageViewSVG>().await;

    check_colors("").await?;

    record_ui_test().await;

    Ok(())
}
