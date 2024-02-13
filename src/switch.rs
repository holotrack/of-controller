use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum State {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Timer {
    seconds: u32,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PortCard {
    state: State,
    duration: Option<Timer>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SwitchCard {
    port_0: PortCard,
    port_1: PortCard,
    port_2: PortCard,
    port_3: PortCard,
    port_4: PortCard,
    port_5: PortCard,
}

impl SwitchCard {
    pub fn new() -> Self {
        SwitchCard {
            port_0: PortCard {
                state: State::Off,
                duration: None,
            },
            port_1: PortCard {
                state: State::On,
                duration: None,
            },
            port_2: PortCard {
                state: State::Off,
                duration: None,
            },
            port_3: PortCard {
                state: State::Off,
                duration: None,
            },
            port_4: PortCard {
                state: State::On,
                duration: None,
            },
            port_5: PortCard {
                state: State::Off,
                duration: None,
            },
        }
    }
}
