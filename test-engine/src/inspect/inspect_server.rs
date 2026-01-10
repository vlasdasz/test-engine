#![cfg(not_wasm)]

use anyhow::Result;
use audio::Sound;
use hreads::on_main;
use inspect::{AppCommand, InspectorCommand, PORT_RANGE, UIRequest, UIResponse};
use log::{debug, error, info};
use refs::manage::DataManager;
use tokio::{spawn, sync::OnceCell};
use ui::UIManager;

use crate::inspect::view_conversion::ViewToInspect;

type Server = netrun::Server<InspectorCommand, AppCommand>;

static SERVER: OnceCell<Server> = OnceCell::const_new();

async fn server() -> &'static Server {
    SERVER
        .get_or_init(|| async {
            let server = Server::new(PORT_RANGE.start).await.unwrap();

            debug!("Inspect server listening on port: {}", PORT_RANGE.start);

            server
        })
        .await
}

pub struct InspectServer {}

impl InspectServer {
    pub fn start_listening() {
        spawn(async {
            server().await.start().await;
            server().await.on_receive(Self::on_receive).await;
        });
    }

    pub fn send(command: AppCommand) {
        spawn(async {
            if let Err(err) = server().await.send(command).await {
                error!("Failed to send app command: {err}");
            }
        });
    }

    fn on_receive(command: InspectorCommand) {
        spawn(async move {
            let command_description = format!("{command:?}");

            if let Err(err) = Self::process_command(command).await {
                error!("Failed to process inspector command: {command_description}. Error: {err}");
            }
        });
    }

    async fn process_command(command: InspectorCommand) -> Result<()> {
        match command {
            InspectorCommand::Ping => {
                server().await.send(AppCommand::Pong).await?;
            }
            InspectorCommand::Pong => {
                info!("Received pong from inspector");
            }
            InspectorCommand::PlaySound => {
                on_main(|| {
                    Sound::get("retro.wav").play();
                });
            }
            InspectorCommand::UI(ui) => Self::process_ui_command(ui).await?,
        }

        Ok(())
    }

    async fn process_ui_command(command: UIRequest) -> Result<()> {
        match command {
            UIRequest::GetScale => {
                server().await.send(UIResponse::Scale(UIManager::scale())).await?;
            }
            UIRequest::SetScale(scale) => {
                UIManager::set_scale(scale);
            }
            UIRequest::GetUI => {
                let root = UIManager::root_view().view_to_inspect();
                server().await.send(UIResponse::SendUI(root)).await?;
            }
        }

        Ok(())
    }
}
