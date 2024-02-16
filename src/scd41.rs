use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Measurements {
    pub cotwo: u16,
    pub temp: f32,
    pub humdt: f32,
}
