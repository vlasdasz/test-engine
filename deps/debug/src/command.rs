use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub enum Command {
    #[default]
    Nothing,
}
