use async_std::task;
use bincode::ErrorKind;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use of_controller::switch;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use rumqttc::v5::{Event, Incoming};
use rumqttc::Outgoing::Publish;
use rumqttc::Request;

use std::error::Error;

use postcard::{from_bytes, to_stdvec};
use std::convert::{Into, TryFrom};
use std::thread;
use std::time::{Duration, SystemTime};

extern crate of_controller;
use crate::of_controller::switch::SwitchCard;

// impl Measurments {
//     fn serialize_postcard(&self) -> Vec<u8> {
//         to_stdvec(self).unwrap()
//     }
// }

#[async_std::main]
async fn main() {
    pretty_env_logger::init();
    // color_backtrace::install();

    let mut mqttoptions = MqttOptions::new("test-1", "mqtt.local", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("sensor_0", QoS::AtMostOnce).await.unwrap();
    task::spawn(async move {
        requests(client).await;
        task::sleep(Duration::from_secs(3)).await;
    });

    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => match v {
                Event::Incoming(Incoming::Publish(pay)) => {
                    println!("Incoming: {:?}", pay.payload)
                }
                Event::Incoming(inc) => println!("Incomint any: {:?}", inc),
                Event::Outgoing(out) => println!("Outgoing: {:?}", out),
            },
            Err(e) => {
                println!("Error = {e:?}");
                return ();
            }
        }
    }
}

async fn requests(client: AsyncClient) {
    loop {
        let switch_card = SwitchCard::new();
        println!("SwitchCard: {:?}", switch_card);

        client
            .publish(
                "switch_0",
                QoS::ExactlyOnce,
                false,
                to_stdvec(&switch_card).unwrap(),
            )
            .await
            .unwrap();

        task::sleep(Duration::from_secs(1)).await;
    }
}
