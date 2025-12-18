use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum InspectorCommand {
    Ping,
    PlaySound,
    UI(UIRequest),
}

impl From<UIRequest> for InspectorCommand {
    fn from(value: UIRequest) -> Self {
        Self::UI(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UIRequest {
    GetScale,
    SetScale(f32),
}
