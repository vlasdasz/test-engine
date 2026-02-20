use anyhow::{Ok, Result};
use test_engine::{
    dispatch::{from_main, on_main, spawn},
    gm::LossyConvert,
    refs::{Weak, manage::DataManager},
    ui::{
        AlertErr, CLEAR, Container, Image, LIGHT_BLUE, Label, ProgressView, Setup, Spinner, UIManager,
        ViewData, ViewSubviews, ViewTest, view_test,
    },
    ui_test::record_ui_test,
};

use crate::interface::test_game_view::TestGameView;

#[view_test]
pub struct LoadingView {
    #[init]
    spinner:  Container,
    label:    Label,
    progress: ProgressView,
}

impl Setup for LoadingView {
    fn setup(self: Weak<Self>) {
        self.spinner.place().center().size(200, 200);

        self.label
            .set_text("Loading...")
            .set_color(CLEAR)
            .place()
            .above(self.spinner, 20)
            .h(40);

        self.progress.place().lrb(0).h(20);

        let mut spinner = self.spinner.add_view::<Spinner>();
        spinner.place().back();
        spinner.dot_color = LIGHT_BLUE;

        spawn(async move {
            self.load(vec![
                "board.png",
                "shop.png",
                "stone_floor.png",
                "triangle.png",
                "sky.png",
                "square.png",
                "bullet.png",
                "cat.png",
                "crate_box.png",
            ])
            .await
            .alert_err();
        });
    }
}

impl LoadingView {
    async fn load(self: Weak<Self>, assets: Vec<&str>) -> Result<()> {
        let part = 1.0 / assets.len().lossy_convert();

        for asset in assets {
            Self::load_asset(asset.to_owned()).await?;
            on_main(move || {
                self.progress.inc_progress(part);
            });
        }

        UIManager::set_view(TestGameView::new());

        Ok(())
    }

    #[allow(clippy::unused_async)]
    async fn load_asset(path: String) -> Result<()> {
        from_main(move || {
            Image::get(path);
        });

        Ok(())
    }
}

impl ViewTest for LoadingView {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        record_ui_test();

        Ok(())
    }
}
