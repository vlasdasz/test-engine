#![cfg(not_wasm)]

use anyhow::Result;
use audio::Sound;
use hreads::{log_spawn, on_main};
use inspect::{AppCommand, InspectorCommand, PORT_RANGE, SystemInfo, SystemResponse, UIRequest, UIResponse};
use log::debug;
use netrun::Service;
use refs::manage::DataManager;
use ui::UIManager;

use crate::inspect::view_conversion::ViewToInspect;

type Server = netrun::Server<InspectorCommand, AppCommand>;

#[derive(Clone)]
pub struct InspectService;

impl Service<InspectorCommand, AppCommand> for InspectService {
    async fn respond(&self, i: InspectorCommand) -> Result<AppCommand> {
        Ok(Self::process_command(i))
    }
}

impl InspectService {
    pub fn start_listening() {
        log_spawn(async {
            let server = Server::start(PORT_RANGE.start).await?;
            debug!("Inspect server listening on: {}", PORT_RANGE.start);
            server.serve(InspectService).await?;

            Ok(())
        });
    }

    fn process_command(command: InspectorCommand) -> AppCommand {
        match command {
            InspectorCommand::PlaySound => {
                on_main(|| {
                    Sound::get("retro.wav").play();
                });

                AppCommand::Ok
            }
            InspectorCommand::UI(ui) => Self::process_ui_command(ui),
            InspectorCommand::GetSystemInfo => SystemResponse::Info(SystemInfo {
                app_id: UIManager::app_instance_id().to_string(),
                info:   netrun::System::get_info(),
            })
            .into(),
        }
    }

    fn process_ui_command(command: UIRequest) -> AppCommand {
        match command {
            UIRequest::GetScale => UIResponse::Scale(UIManager::scale()).into(),
            UIRequest::SetScale(scale) => {
                UIManager::set_scale(scale);
                AppCommand::Ok
            }
            UIRequest::GetUI => {
                let root = UIManager::root_view().view_to_inspect();
                UIResponse::SendUI(root).into()
            }
        }
    }
}
