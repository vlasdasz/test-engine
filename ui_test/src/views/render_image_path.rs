use anyhow::Result;
use log::debug;
use test_engine::{gl_wrapper::path_data::DrawMode, gm::Color, Image, Screen};
use ui::{refs::Weak, view, SubView, ViewSetup};
use ui_views::ImageView;

use crate::view_tests::record_touches;

#[view]
struct RenderImagePath {
    image_view: SubView<ImageView>,
}

impl ViewSetup for RenderImagePath {
    fn setup(mut self: Weak<Self>) {
        self.image_view.place.back();
        self.image_view.image = Image::render_path(
            "test_path",
            Color::BLUE,
            vec![(1, 0).into(), (1, 50).into(), (50, 50).into(), (15, 35).into()],
            DrawMode::Outline,
        );
    }
}

pub async fn test_render_image_path() -> Result<()> {
    Screen::set_test_view::<RenderImagePath>(400, 400).await;

    record_touches().await;

    debug!("Image view test: OK");

    Ok(())
}
