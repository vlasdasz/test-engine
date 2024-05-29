use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Label, ViewData, ViewSetup, UI},
};

#[view]
struct MultilineTestView {
    label: Label,
}

impl ViewSetup for MultilineTestView {
    fn setup(mut self: Weak<Self>) {
        self.label.place().back();
        self.label.set_text(
            "       Plati mne dengi bistrenko pliz. Ja kuplu dengushki.\n      Plati mne dengi bistrenko \
             pliz. Ja kuplu dengushki.",
        );
    }
}

pub async fn test_multiline() -> Result<()> {
    UI::init_test_view::<MultilineTestView>().await;

    debug!("Multiline test: OK");

    Ok(())
}
