#![cfg(not_wasm)]

use std::ops::DerefMut;

use anyhow::Result;
use audio::Sound;
use hreads::{log_spawn, on_main};
use inspect::{AppCommand, InspectorCommand, PORT_RANGE, SystemInfo, SystemResponse, UIRequest, UIResponse};
use log::{debug, error, info};
use parking_lot::Mutex;
use refs::manage::DataManager;
use tokio::{
    spawn,
    sync::{OnceCell, RwLock, RwLockReadGuard},
};
use ui::UIManager;

use crate::inspect::view_conversion::ViewToInspect;

type Server = netrun::Server<InspectorCommand, AppCommand>;
type Client = netrun::Client<InspectorCommand, AppCommand>;

static SERVER: OnceCell<Server> = OnceCell::const_new();
static CONNECTION: OnceCell<RwLock<Client>> = OnceCell::const_new();

async fn server() -> Result<&'static Server> {
    SERVER
        .get_or_try_init(|| async {
            let server = Server::new(PORT_RANGE.start).await?;

            debug!("Inspect server listening on port: {}", PORT_RANGE.start);

            Ok(server)
        })
        .await
}

pub struct InspectServer {}

impl InspectServer {
    async fn current_connection() -> RwLockReadGuard<'static, Client> {
        CONNECTION.get().unwrap().read().await
    }

    pub fn start_listening() {
        spawn(async {
            loop {
                let client = server().await.expect("Failed to get server").wait_for_new_connection().await;

                if let Some(conn) = CONNECTION.get() {
                    *conn.write().await = client;
                } else {
                    CONNECTION.set(RwLock::new(client)).unwrap();
                }

                log_spawn(async move {
                    loop {
                        Self::on_receive(Self::current_connection().await.receive().await?);
                    }

                    Ok(())
                });
            }
        });
    }

    pub async fn send(command: impl Into<AppCommand>) -> Result<()> {
        Self::current_connection().await.send(command).await
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
                Self::send(AppCommand::Pong).await?;
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
            InspectorCommand::GetSystemInfo => {
                Self::send(SystemResponse::Info(SystemInfo {
                    app_id: UIManager::app_instance_id().to_string(),
                    info:   netrun::System::get_info(),
                }))
                .await?;
            }
        }

        Ok(())
    }

    async fn process_ui_command(command: UIRequest) -> Result<()> {
        match command {
            UIRequest::GetScale => {
                Self::send(UIResponse::Scale(UIManager::scale())).await?;
            }
            UIRequest::SetScale(scale) => {
                UIManager::set_scale(scale);
            }
            UIRequest::GetUI => {
                let root = UIManager::root_view().view_to_inspect();
                Self::send(UIResponse::SendUI(root)).await?;
            }
        }

        Ok(())
    }
}
