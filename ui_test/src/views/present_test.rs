use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, Color, Container, NavigationView, ViewController, ViewData, ViewSetup},
    App,
};

use crate::view_tests::record_touches;

#[view]
struct PresentTestView {}

impl ViewSetup for PresentTestView {
    fn setup(self: Weak<Self>) {}
}

pub async fn test_present() -> Result<()> {
    let present = PresentTestView::new();

    let view = present.weak();

    let _navigation = App::set_test_view(NavigationView::with_view(present), 600, 600).await;

    from_main(move || {
        let mut presented = Container::new();
        presented.set_color(Color::RED);

        view.present(presented);
    })
    .await;

    record_touches().await;

    debug!("Present test: OK");

    Ok(())
}
