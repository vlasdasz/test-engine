use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum LevelCommand {
    SetScale(f32),
    GetScale,
    SendScale(f32),
    DoSomethingThatIsNotThis,
}

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub enum Command {
    #[default]
    Ping,
    Level(LevelCommand),
}

impl From<LevelCommand> for Command {
    fn from(value: LevelCommand) -> Self {
        Self::Level(value)
    }
}
