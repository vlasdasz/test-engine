use anyhow::Result;
use test_engine::ui::{view, MovableView, UI};

#[view]
struct MovableViewTestView {
    movable: MovableView,
}

pub async fn test_movable_view() -> Result<()> {
    let mut _view = UI::init_test_view::<MovableViewTestView>().await;

    Ok(())
}
