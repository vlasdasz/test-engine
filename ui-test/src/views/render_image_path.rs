use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, ImageView, Sub, ViewData, ViewSetup, UI},
    ui_test::record_touches,
};

#[view]
struct RenderImagePath {
    image_view: Sub<ImageView>,
}

impl ViewSetup for RenderImagePath {
    fn setup(self: Weak<Self>) {
        self.image_view.place().back();
        // self.image_view.image = Image::render_path(
        //     "test_path",
        //     Color::BLUE,
        //     vec![(1, 0).into(), (1, 50).into(), (50, 50).into(), (15,
        // 35).into()],     DrawMode::Outline,
        // );
    }
}

pub async fn test_render_image_path() -> Result<()> {
    UI::init_test_view::<RenderImagePath>().await;

    record_touches().await;

    debug!("Image view test: OK");

    Ok(())
}
