use anyhow::Result;
use log::debug;
use test_engine::Screen;
use ui::{refs::Weak, view, SubView, ViewData, ViewSetup};
use ui_views::MultilineLabel;

#[view]
struct MultilineTestView {
    label: SubView<MultilineLabel>,
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
    Screen::set_test_view::<MultilineTestView>(600, 600).await;

    debug!("Multiline test: OK");

    Ok(())
}
