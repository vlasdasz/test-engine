use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Anchor::Left, BLUE, Container, ImageView, Setup, UIDrawer, ViewData, YELLOW, view},
    ui_test::{check_colors, record_ui_test},
};

#[view]
struct Outline {
    #[init]
    square: Container,
    image:  ImageView,
    wide:   Container,
}

impl Setup for Outline {
    fn setup(mut self: Weak<Self>) {
        self.square.set_color(BLUE);
        self.square.place().size(100, 100).tl(50);

        self.image.set_image("cat.png");
        self.image.place().size(100, 200).t(50).anchor(Left, self.square, 20);

        self.wide.set_color(YELLOW);
        self.wide.place().size(200, 100).t(50).anchor(Left, self.image, 20);
    }
}

pub async fn test_outline() -> Result<()> {
    UIDrawer::init_test_view::<Outline>();

    record_ui_test();

    Ok(())
}
