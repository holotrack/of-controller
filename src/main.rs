use async_std::channel::{self, Sender};
use async_std::io::{ReadExt, WriteExt};
use async_std::sync::{Mutex, MutexGuard};
use async_std::{net::TcpStream, task};
use chrono::prelude::*;
use std::sync::Arc;
use std::time::Duration;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use postcard::{from_bytes, to_slice};

use of_controller::controller::SwitchStatus;
use of_controller::scd41::Measurements;
use of_controller::switch::{Message, PortCard};

const PORTS_AMMOUNT: usize = 6;
const FAN_PORT: usize = 0;
const ATOMIZER_PORT: usize = 1;
const HEAT_PORT: usize = 2;
const TIMER_PORT: usize = 3;

const MAX_CO2: u16 = 1000;
// const MIN_CO2: u16 = 500; //not seeing usage of this value

const MAX_TEMP: f32 = 26_f32;
const MIN_TEMP: f32 = 18_f32;

const MAX_HMDT: f32 = 95_f32;
const MIN_HMDT: f32 = 80_f32;

// const LIGHT_ON:

async fn scd41_read(addr: &str, tx: Sender<Measurements>) -> ! {
    loop {
        task::sleep(Duration::from_millis(1000)).await;

        match TcpStream::connect(addr).await {
            Ok(mut stream) => {
                info!("Successfully connected to server in port 1234");

                let mut data = [0u8; 12]; // using 12 byte buffer

                match stream.read(&mut data).await {
                    Ok(_) => {
                        let measurements: Measurements = from_bytes(&data).unwrap();

                        info!("Measurements from sensor: {:?}", measurements);
                        tx.send(measurements).await.unwrap();
                    }

                    Err(e) => {
                        error!("Failed to receive data: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("SCD41 failed to connect: {}", e);
            }
        }
        info!("SCD41 read finished.");
    }
}

async fn switch_status_update(addr: &str, mutex: Arc<Mutex<SwitchStatus>>) {
    let mut buf: [u8; 4096] = [0; 4096];
    let mut read: [u8; 1024] = [0; 1024];

    let mut switch_status = mutex.lock().await;
    for (index, port) in switch_status.ports().iter_mut().enumerate() {
        match TcpStream::connect(addr).await {
            Ok(mut stream) => {
                info!("Successfully connected to server in port 1234");
                debug!("Setting status of port: {} to {:?}", index, port.status());
                let message =
                    Message::GetPortStatus(Some(PortCard::new(index, port.status(), None)));

                let data = to_slice(&message, &mut buf).unwrap();

                match stream.write_all(data).await {
                    Ok(()) => {
                        info!("Written request for pin stats");
                        // task::sleep(Duration::from_millis(1000)).await;
                        //next line never success
                        match stream.read(&mut read).await {
                            Ok(_) => {
                                debug!("Status update recived: {read:?}");
                                let message: Message = postcard::from_bytes(&read).unwrap();
                                if let Message::GetPortStatus(port_card) = message {
                                    if let Some(card) = port_card {
                                        info!(
                                            "Set status on pin: {} to: {:?}",
                                            card.port, card.state
                                        );
                                    } else {
                                        info!("After ask for status of pin recived None");
                                    }

                                    // port.sent();
                                } else {
                                    debug!("IT SHOULD NEVER HAPPEN");
                                    info!("{message:?}");
                                }
                                break;
                            }
                            Err(e) => {
                                error!("Pin remote status read error: {:?}", e);
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Pin status update write error: {:?}", e);
                    }
                };
            }
            Err(e) => {
                error!("Update status failed to connect to switch: {}", e);
            }
        }
        info!("Pin status update finished");
    }
}

async fn swtich_write(addr: &str, mutex: Arc<Mutex<SwitchStatus>>) -> ! {
    loop {
        task::sleep(Duration::from_secs(1)).await;

        let mut buf: [u8; 4096] = [0; 4096];

        let mut switch_status = mutex.lock().await;
        if switch_status.is_updated() {
            for (index, port) in switch_status.ports().iter_mut().enumerate() {
                if port.is_updated() {
                    match TcpStream::connect(addr).await {
                        Ok(mut stream) => {
                            info!("Successfully connected to server in port 1234");
                            debug!("Setting port: {} to {:?}", index, port.status());

                            let message =
                                Message::SetPort(PortCard::new(index, port.status(), None));

                            let data = to_slice(&message, &mut buf).unwrap();

                            match stream.write_all(data).await {
                                Ok(()) => {
                                    if let Message::SetPort(port_card) = message {
                                        info!(
                                            "Switch pin: {} set to: {:?}",
                                            port_card.port, port_card.state
                                        );
                                        port.sent();
                                    } else {
                                        debug!("IT SHOULD NEVER HAPPEN");
                                    }
                                }
                                Err(e) => {
                                    error!("write error: {:?}", e);
                                }
                            };
                        }
                        Err(e) => {
                            error!("Switch failed to connect: {}", e);
                        }
                    }
                    info!("Pin write finished.");
                } else {
                    debug!("Port #{} not for update", index);
                }
            }
        } else {
            info!("Nothing to do with switch pin setup");
        }
    }
}

#[async_std::main]
async fn main() {
    pretty_env_logger::init();

    let switch = SwitchStatus::new(PORTS_AMMOUNT);
    debug!("Switch default: {:?}", switch);

    let switch_mutex: Arc<Mutex<SwitchStatus>> = Arc::new(Mutex::new(switch));

    let (sender, receiver) = channel::unbounded();

    let switch_update_mutex = switch_mutex.clone();
    switch_status_update("192.168.1.138:1234", switch_update_mutex).await;

    let switch_spawn_mutex = switch_mutex.clone();

    task::spawn(async move { scd41_read("192.168.1.154:1234", sender).await });
    task::spawn(async move { swtich_write("192.168.1.138:1234", switch_spawn_mutex).await });

    let switch_main_mutex = switch_mutex.clone();

    loop {
        task::sleep(Duration::from_secs(1)).await;

        let measurments = receiver.recv().await.unwrap();

        info!("Recived: {:?}", measurments);
        let mut switch_status = switch_main_mutex.lock().await;

        let time_on = NaiveTime::from_hms_opt(00, 00, 00).unwrap();
        let time_off = NaiveTime::from_hms_opt(00, 40, 00).unwrap();
        let time_now = Local::now().naive_local().time();

        info!("Now is {time_now} imer set to start lights on {time_on} and turn off at {time_off}");

        if time_now >= time_on && time_now <= time_off {
            switch_status.set_port_on(TIMER_PORT);
        } else {
            switch_status.set_port_off(TIMER_PORT);
        }

        // MAXES - turn on/off can be moved to separate functions
        if measurments.cotwo > MAX_CO2
            || measurments.temp > MAX_TEMP
            || measurments.humdt > MAX_HMDT
        {
            switch_status.set_port_on(FAN_PORT);
        } else {
            switch_status.set_port_off(FAN_PORT);
        }

        // MAXES - turn on/off can be moved to separate functions
        if measurments.temp < MIN_TEMP {
            switch_status.set_port_on(HEAT_PORT);
        } else {
            switch_status.set_port_off(HEAT_PORT);
        }

        if measurments.temp < MIN_HMDT {
            switch_status.set_port_on(ATOMIZER_PORT);
        } else {
            switch_status.set_port_off(ATOMIZER_PORT);
        }
    }
}
