use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub enum Command {
    #[default]
    Ping,
    Level(LevelCommand),
    UI(UICommand),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum UICommand {
    SetScale(f32),
}

impl From<UICommand> for Command {
    fn from(value: UICommand) -> Self {
        Self::UI(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum LevelCommand {
    SetScale(f32),
    GetScale,
    SendScale(f32),
    Panic,
}

impl From<LevelCommand> for Command {
    fn from(value: LevelCommand) -> Self {
        Self::Level(value)
    }
}
