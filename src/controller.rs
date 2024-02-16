use log::info;

use crate::switch::State;

#[derive(Default, Debug)] //Default not fit here (vec is not initialized to be usable) but leave it here to use in future for other things
pub struct SwitchStatus {
    ports: Vec<PortUpdate>,
    updated: bool,
}

impl SwitchStatus {
    pub fn new(port_amount: usize) -> Self {
        Self {
            ports: vec![
                PortUpdate {
                    need_update: false,
                    status: State::Off,
                };
                port_amount
            ],
            updated: false,
        }
    }

    pub fn updated(&mut self) {
        self.updated = true;
    }

    pub fn is_updated(&self) -> bool {
        self.updated
    }

    pub fn ports(&mut self) -> &mut Vec<PortUpdate> {
        &mut self.ports
    }

    pub fn set_port_on(&mut self, port: usize) {
        match self.ports[port].status {
            State::On => info!("Port {port} already setup to On status"),
            State::Off => {
                self.ports[port].status_on();
                self.ports[port].updated();
                self.updated();
                info!("Port {port} set to On status")
            }
        }
    }

    pub fn set_port_off(&mut self, port: usize) {
        match self.ports[port].status {
            State::On => {
                self.ports[port].status_off();
                self.ports[port].updated();
                self.updated();
                info!("Port {port} set to Off status")
            }
            State::Off => info!("Port {port} already setup to Off status"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PortUpdate {
    need_update: bool,
    status: State,
}

impl PortUpdate {
    pub fn updated(&mut self) {
        self.need_update = true;
    }

    pub fn is_updated(&self) -> bool {
        self.need_update
    }

    pub fn sent(&mut self) {
        self.need_update = false;
    }

    pub fn status(&self) -> State {
        self.status
    }

    pub fn status_on(&mut self) {
        self.status = State::On;
    }

    pub fn status_off(&mut self) {
        self.status = State::Off;
    }
}
