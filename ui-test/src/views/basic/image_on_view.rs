use anyhow::Result;
use log::debug;
use test_engine::{
    App,
    refs::Weak,
    ui::{Anchor, Color, Container, ImageView, Setup, UI, UIImages, ViewData, ViewSubviews, ViewTouch, view},
    ui_test::{helpers::check_colors, record_ui_test},
};

#[view]
struct ImageOnViewTestView {
    image: Weak<ImageView>,

    #[init]
    container: Container,
}

impl Setup for ImageOnViewTestView {
    fn setup(mut self: Weak<Self>) {
        self.container.set_color(Color::GREEN).place().size(200, 200).center();

        self.image = self.container.add_view();

        self.image.set_image(UIImages::rb()).place().size(100, 100).center();
    }
}

pub async fn test_image_on_view() -> Result<()> {
    debug!("Image on view:");

    UI::init_test_view::<ImageOnViewTestView>().await;

    App::set_window_size((400, 400)).await;

    record_ui_test().await;

    debug!("OK");

    Ok(())
}
