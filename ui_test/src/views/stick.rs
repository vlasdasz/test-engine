use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{view, PointView, StickView, SubView, ViewData, ViewFrame, ViewSetup},
    App,
};

use crate::view_tests::record_ui_test;

#[view]
struct StickTestView {
    stick: SubView<StickView>,
    pos:   SubView<PointView>,
}

impl ViewSetup for StickTestView {
    fn setup(mut self: Weak<Self>) {
        self.stick.set_size((200, 200));
        self.pos.set_multiplier(20).place().size(200, 200).bl(0);
        self.pos.changed.val(move |pos| {
            self.stick.set_origin(pos);
        });
    }
}

pub async fn test_stick() -> Result<()> {
    App::init_test_view::<StickTestView>(600, 600).await;

    record_ui_test().await?;

    Ok(())
}
