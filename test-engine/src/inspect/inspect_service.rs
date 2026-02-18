#![cfg(not_wasm)]

use audio::Sound;
use hreads::{log_spawn, on_main};
use inspect::{AppCommand, InspectorCommand, SystemInfo, SystemResponse, UIRequest, UIResponse};
use netrun::zmq::Rep;
use refs::manage::DataManager;
use ui::UIManager;

use crate::inspect::view_conversion::ViewToInspect;

#[derive(Clone)]
pub struct InspectService;

impl InspectService {
    pub fn start_listening() {
        log_spawn(async {
            let server = Rep::<InspectorCommand, AppCommand>::new("tcp://0.0.0.0:6969").await?;
            server.on_receive(|command| Self::process_command(command));
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
                let scale = UIManager::scale();
                let root = UIManager::root_view().view_to_inspect();
                UIResponse::SendUI { scale, root }.into()
            }
        }
    }
}
