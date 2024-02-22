use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, IntView, Label, SubView, ViewData, ViewFrame, ViewSetup},
    App,
};

use crate::view_tests::record_touches;

#[view]
struct OutBoundsView {
    test: SubView<Label>,
    x: SubView<IntView>,
    y: SubView<IntView>,
}

impl ViewSetup for OutBoundsView {
    fn setup(mut self: Weak<Self>) {
        self.test.set_text("AA").set_text_size(100).set_frame((200, 200, 200, 200));
        self.x.set_step(25);
        self.x
            .on_change(move |val| {
                self.test.set_x(200.0 + val);
            })
            .place()
            .size(60, 200)
            .bl(0);

        self.y.set_step(25);
        self.y
            .on_change(move |val| {
                self.test.set_y(200.0 + val);
            })
            .place()
            .size(60, 200)
            .b(0)
            .l(100);
    }
}

pub async fn test_out_bounds() -> Result<()> {
    App::init_test_view::<OutBoundsView>(600, 600).await;

    record_touches().await;

    debug!("Out bounds test: OK");

    Ok(())
}
