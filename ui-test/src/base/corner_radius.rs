use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Anchor::Left, Color, Container, Setup, ViewData, UI},
};

#[view]
struct CornerRadiusTestView {
    #[init]
    square: Container,
    tall:   Container,
    wide:   Container,
}

impl Setup for CornerRadiusTestView {
    fn setup(mut self: Weak<Self>) {
        self.square.set_color(Color::BLUE).set_corner_radius(10);
        self.square.place().size(100, 100).tl(10);

        self.tall.set_color(Color::GREEN).set_corner_radius(10);
        self.tall.place().size(100, 200).t(10).anchor(Left, self.square, 10);

        self.wide.set_color(Color::YELLOW).set_corner_radius(10);
        self.wide.place().size(200, 100).t(10).anchor(Left, self.tall, 10);
    }
}

pub async fn test_corner_radius() -> Result<()> {
    UI::init_test_view::<CornerRadiusTestView>().await;

    // record_ui_test().await;

    debug!("Corner radius test: OK");

    Ok(())
}
