use anyhow::Result;
use inspect::{AppCommand, InspectorCommand, UIRequest, UIResponse};
use log::info;
use test_engine::{
    dispatch::{after, on_main},
    refs::Weak,
    ui::{
        AlertErr,
        Anchor::{Right, Top},
        Button, HasText, Label, NumberView, Setup, ViewData, async_link_button, view,
    },
};
use tokio::spawn;

use crate::app_search::client;

#[view]
pub struct MainScreen {
    #[init]
    play_sound:   Button,
    scale_picker: NumberView,
    scale_label:  Label,
}

impl Setup for MainScreen {
    fn setup(mut self: Weak<Self>) {
        self.play_sound.set_text("Play Sound").place().size(100, 50).tr(10);
        async_link_button!(self.play_sound, play_sound_tapped);

        self.scale_picker.place().r(10).anchor(Top, self.play_sound, 10).size(50, 100);
        self.scale_picker.set_min(0.2);
        self.scale_picker.set_step(0.2);
        self.scale_picker.on_change(move |val| {
            spawn(async move {
                self.scale_tapped(val).await.alert_err();
            });
        });

        self.scale_label
            .place()
            .anchor(Top, self.play_sound, 10)
            .anchor(Right, self.scale_picker, 10)
            .same_size(self.scale_picker);

        spawn(async move {
            client()
                .await
                .on_receive(move |command| {
                    spawn(async move {
                        self.process_command(command).await.alert_err();
                    });
                })
                .await;
        });

        after(0.5, || {
            spawn(async {
                client().await.send(UIRequest::GetScale).await.alert_err();
            });
        });
    }
}

impl MainScreen {
    async fn play_sound_tapped(self: Weak<Self>) -> Result<()> {
        client().await.send(InspectorCommand::PlaySound).await
    }

    async fn scale_tapped(mut self: Weak<Self>, scale: f32) -> Result<()> {
        on_main(move || {
            self.scale_label.set_text(scale);
        });
        client().await.send(UIRequest::SetScale(scale)).await
    }

    async fn process_command(self: Weak<Self>, command: AppCommand) -> Result<()> {
        match command {
            AppCommand::Ping => {
                client().await.send(InspectorCommand::Pong).await?;
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

    async fn process_ui_command(mut self: Weak<Self>, command: UIResponse) -> Result<()> {
        match command {
            UIResponse::Scale(scale) => {
                on_main(move || {
                    self.scale_label.set_text(scale);
                    self.scale_picker.set_value(scale);
                });
            }
        };

        Ok(())
    }
}
