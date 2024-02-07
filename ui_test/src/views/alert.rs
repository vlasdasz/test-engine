use anyhow::Result;
use log::debug;
use old_engine::Screen;
use ui::{refs::Weak, view, ViewSetup};

#[view]
struct AlertTestView {}

impl ViewSetup for AlertTestView {
    fn setup(self: Weak<Self>) {}
}

pub async fn test_alert() -> Result<()> {
    Screen::set_test_view::<AlertTestView>(600, 600).await;

    // let answer = Question::ask_async(
    //     "Plati mne dengi bistrenko pliz. Ja kuplu dengushki.\n Plati mne dengi
    // bistrenko pliz. Ja kuplu \      dengushki.",
    // )
    // .await;
    //
    // dbg!(answer);

    debug!("Alert test: OK");

    Ok(())
}
