use std::time::Duration;

use anyhow::Result;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{Setup, UI, UIManager, ViewCallbacks, ViewData, ViewTouch, WHITE, view},
    ui_test::{helpers::check_colors, record_ui_test},
};
use tokio::time::sleep;

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

impl ViewCallbacks for ImageViewSVG {
    fn update(&mut self) {
        // dbg!(UIManager::window_resolution());
    }
}

pub async fn test_image_view_svg() -> Result<()> {
    from_main(|| {
        UIManager::set_clear_color(WHITE);

        dbg!(UIManager::window_resolution());
        dbg!(UIManager::display_scale());
    })
    .await;

    let _view = UI::init_test_view::<ImageViewSVG>().await;

    check_colors("").await?;

    sleep(Duration::from_secs(50000)).await;

    record_ui_test().await;

    Ok(())
}
