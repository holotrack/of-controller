use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Default)]
pub enum State {
    On,
    #[default]
    Off,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Timer {
    seconds: u32,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PortCard {
    pub port: usize,
    pub state: State,
    pub duration: Option<Timer>,
}

impl PortCard {
    pub fn new(port: usize, state: State, duration: Option<Timer>) -> Self {
        PortCard {
            port,
            state,
            duration,
        }
    }
}
