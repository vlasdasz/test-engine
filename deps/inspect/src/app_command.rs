use serde::{Deserialize, Serialize};

use crate::ui::ViewRepr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppCommand {
    Ok,
    UI(UIResponse),
    System(SystemResponse),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIResponse {
    Scale(f32),
    SendUI { scale: f32, root: ViewRepr },
}

impl From<UIResponse> for AppCommand {
    fn from(value: UIResponse) -> Self {
        Self::UI(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub app_id: String,
    pub info:   netrun::System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemResponse {
    Info(SystemInfo),
}

impl From<SystemResponse> for AppCommand {
    fn from(value: SystemResponse) -> Self {
        Self::System(value)
    }
}
