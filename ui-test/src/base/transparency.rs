use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    gm::Apply,
    level::LevelManager,
    refs::Weak,
    ui::{view, ImageView, Setup, ViewData, UI},
};

use crate::level::SkyboxLevel;

#[view]
struct TransparencyTestView {
    #[init]
    background: ImageView,

    view_1: ImageView,
    view_2: ImageView,
    view_3: ImageView,
    view_4: ImageView,
}

impl Setup for TransparencyTestView {
    fn setup(mut self: Weak<Self>) {
        self.background.set_image("gradient.png").place().back();

        self.view_1.set_image("wood-window.png");
        self.view_2.set_image("wood-window.png").place().tl(50);
        self.view_3.set_image("wood-window.png").place().tl(100);
        self.view_4.set_image("wood-window.png").place().tl(150);

        [self.view_1, self.view_2, self.view_3, self.view_4].apply(|v| {
            v.place().size(280, 280);
        });
    }
}

pub async fn test_transparency() -> Result<()> {
    UI::init_test_view::<TransparencyTestView>().await;

    from_main(|| {
        LevelManager::set_level(SkyboxLevel::default());
    })
    .await;

    from_main(|| {
        LevelManager::stop_level();
    })
    .await;

    debug!("Transparency test: OK");

    Ok(())
}
