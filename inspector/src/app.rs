use test_engine::{
    App,
    refs::Own,
    ui::{Setup, Size, View},
};
use tokio::spawn;

use crate::{app_search::client, ui::MainScreen};

pub struct InspectorApp;

impl App for InspectorApp {
    fn new() -> Box<Self>
    where Self: Sized {
        Box::new(Self)
    }

    fn after_launch(&self) {
        spawn(async {
            client().await.start().await;
        });
    }

    fn make_root_view(&self) -> Own<dyn View> {
        MainScreen::new()
    }

    fn initial_size(&self) -> Size {
        (800, 800).into()
    }

    fn enable_inspection(&self) -> bool {
        false
    }
}
