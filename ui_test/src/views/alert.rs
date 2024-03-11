use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, ViewSetup},
    App,
};

#[view]
struct AlertTestView {}

impl ViewSetup for AlertTestView {
    fn setup(self: Weak<Self>) {}
}

pub async fn test_alert() -> Result<()> {
    App::init_test_view::<AlertTestView>().await;

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
