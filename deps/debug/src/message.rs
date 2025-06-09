use serde::{Deserialize, Serialize};

use crate::command::Command;

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
