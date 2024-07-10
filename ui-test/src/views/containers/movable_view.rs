use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Color, Container, HasTitle, MovableView, ViewData, ViewFrame, ViewSetup, UI},
};

#[view]
struct MovableViewTestView {
    #[init]
    movable: MovableView<Container>,
}

impl ViewSetup for MovableViewTestView {
    fn setup(mut self: Weak<Self>) {
        self.movable.set_title("Movable view");
        self.movable.set_frame((10, 10, 400, 400));
        self.movable.target_view.set_color(Color::GREEN);
    }
}

pub async fn test_movable_view() -> Result<()> {
    let mut _view = UI::init_test_view::<MovableViewTestView>().await;

    // record_ui_test().await;

    debug!("Test movable view: OK");

    Ok(())
}
