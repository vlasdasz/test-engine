use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Button, HasText, Setup, Spinner, ViewData, async_link_button, view},
};

#[view]
pub struct MainScreen {
    #[init]
    search: Button,
}

impl MainScreen {
    async fn search_tapped(self: Weak<Self>) -> Result<()> {
        let _spin = Spinner::lock();

        Ok(())
    }
}

impl Setup for MainScreen {
    fn setup(mut self: Weak<Self>) {
        self.search.set_text("Search").place().size(100, 50).tr(10);

        async_link_button!(self.search, search_tapped);
    }
}
