use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Anchor, IntView, Label, SubView, ViewData, ViewSetup},
    App,
};

#[view]
struct LabelTestView {
    label:          SubView<Label>,
    text_size_view: SubView<IntView>,
}

impl Drop for LabelTestView {
    fn drop(&mut self) {
        dbg!("drebatol");
    }
}

impl ViewSetup for LabelTestView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("ßšėčыў");
        self.label.place().back().size(280, 280).center();

        self.text_size_view
            .place()
            .size(50, 100)
            .center_y()
            .anchor(Anchor::Right, self.label, 10);
        self.text_size_view.set_value(32).set_step(5);

        self.text_size_view.on_change(move |size| {
            self.label.set_text_size(size);
        });
    }
}

pub async fn test_label() -> Result<()> {
    App::set_test_view::<LabelTestView>(400, 400).await;

    debug!("Label test: OK");

    Ok(())
}
