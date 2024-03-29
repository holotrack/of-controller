use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Orders {
    Status(Option<(bool, bool)>),
    Order(Command),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Command {
    Reset,
    FirmwareUpdate,
}
