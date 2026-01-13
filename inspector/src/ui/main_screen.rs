use anyhow::Result;
use inspect::{AppCommand, InspectorCommand, UIRequest, UIResponse};
use log::{error, info};
use test_engine::{
    dispatch::{after, on_main},
    refs::Weak,
    ui::{AlertErr, Anchor::Top, Button, HasText, Setup, ViewData, async_link_button, view},
};
use tokio::spawn;

use crate::{app_search::client, ui::common::ValueView};

#[view]
pub struct MainScreen {
    #[init]
    play_sound:     Button,
    get_ui:         Button,
    ui_scale_value: ValueView,
}

impl Setup for MainScreen {
    fn setup(mut self: Weak<Self>) {
        self.play_sound.set_text("Play Sound").place().size(100, 50).tr(10);
        async_link_button!(self.play_sound, play_sound_tapped);

        self.get_ui.set_text("Get UI");
        self.get_ui.place().below(self.play_sound, 10);
        async_link_button!(self.get_ui, get_ui_tapped);

        self.ui_scale_value
            .set_title("UI scale")
            .place()
            .r(10)
            .anchor(Top, self.get_ui, 10)
            .size(100, 100);

        self.ui_scale_value.on_change.val_async(move |val| async move {
            {
                self.scale_changed(val).await.alert_err();
            }
        });

        spawn(async move {
            loop {
                if let Some(msg) = client().await.unwrap().receive().await {
                    spawn(async move {
                        self.process_command(msg).await.alert_err();
                    });
                } else {
                    error!("Client received None")
                }
            }
        });

        after(0.5, || {
            spawn(async {
                client().await.unwrap().send(UIRequest::GetScale).await.alert_err();
            });
        });
    }
}

impl MainScreen {
    async fn play_sound_tapped(self: Weak<Self>) -> Result<()> {
        client().await?.send(InspectorCommand::PlaySound).await
    }

    async fn get_ui_tapped(self: Weak<Self>) -> Result<()> {
        client().await?.send(UIRequest::GetUI).await
    }

    async fn scale_changed(self: Weak<Self>, scale: f32) -> Result<()> {
        client().await?.send(UIRequest::SetScale(scale)).await
    }

    async fn process_command(self: Weak<Self>, command: AppCommand) -> Result<()> {
        match command {
            AppCommand::Ping => {
                client().await?.send(InspectorCommand::Pong).await?;
            }
            AppCommand::Pong => {
                info!("Received pong from the app");
            }
            AppCommand::UI(ui) => {
                self.process_ui_command(ui).await?;
            }
        };

        Ok(())
    }

    async fn process_ui_command(self: Weak<Self>, command: UIResponse) -> Result<()> {
        match command {
            UIResponse::Scale(scale) => {
                on_main(move || {
                    self.ui_scale_value.set_value(scale);
                });
            }
            UIResponse::SendUI(root) => {
                dbg!(&root);
            }
        };

        Ok(())
    }
}
