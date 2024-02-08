use async_std::task;
use bincode::ErrorKind;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use rumqttc::v5::{Event, Incoming};
use rumqttc::Outgoing::Publish;
use rumqttc::Request;
use serde::{Deserialize, Serialize};
use std::error::Error;

use std::convert::{Into, TryFrom};
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Debug)]
struct Measurments {
    cotwo: u16,
    temp: f32,
    humdt: f32,
}

impl From<&Measurments> for Bytes {
    fn from(value: &Measurments) -> Self {
        bincode::serialize(value).unwrap().into()
    }
}

impl From<Measurments> for Bytes {
    fn from(value: Measurments) -> Self {
        bincode::serialize(&value).unwrap().into()
    }
}

impl TryFrom<&[u8]> for Measurments {
    type Error = Box<ErrorKind>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        bincode::deserialize(value)
    }
}

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
        let meas = Measurments {
            cotwo: 11,
            temp: 2.2,
            humdt: 3.3,
        };

        client
            .publish("sensor_0", QoS::ExactlyOnce, false, meas)
            .await
            .unwrap();

        task::sleep(Duration::from_secs(1)).await;
    }
}
