use anyhow::Result;
use inspect::InspectorCommand;
use test_engine::{
    refs::Weak,
    ui::{Button, HasText, Setup, ViewData, async_link_button, view},
};

use crate::app_search::client;

#[view]
pub struct MainScreen {
    #[init]
    play_sound: Button,
}

impl MainScreen {
    async fn play_sound_tapped(self: Weak<Self>) -> Result<()> {
        client().await.send(InspectorCommand::PlaySound).await
    }
}

impl Setup for MainScreen {
    fn setup(mut self: Weak<Self>) {
        self.play_sound.set_text("Play Sound").place().size(100, 50).tr(10);

        async_link_button!(self.play_sound, play_sound_tapped);
    }
}
