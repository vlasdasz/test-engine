use serde::{Deserialize, Serialize};

use crate::command::{Command, LevelCommand};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DebugMessage {
    pub id:      u32,
    pub msg:     String,
    pub command: Command,
}

impl From<Command> for DebugMessage {
    fn from(command: Command) -> Self {
        Self {
            id: 5,
            msg: "command".to_string(),
            command,
        }
    }
}

impl From<LevelCommand> for DebugMessage {
    fn from(value: LevelCommand) -> Self {
        Command::from(value).into()
    }
}
