use serde::{Deserialize, Serialize};

use crate::ui::ViewRepr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppCommand {
    Ping,
    Pong,
    UI(UIResponse),
}

impl From<UIResponse> for AppCommand {
    fn from(value: UIResponse) -> Self {
        Self::UI(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIResponse {
    Scale(f32),
    SendUI(ViewRepr),
}
