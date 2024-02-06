use async_std::io::{Read, ReadExt, Write};
use async_std::net::{Ipv4Addr, TcpStream};
use async_std::os::unix::io::AsRawFd;
use async_std::task;
use rust_mqtt::client::client::MqttClient;
use std::str::from_utf8;

use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use core::ops::Deref;
use postcard::{from_bytes, to_slice};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Measurments {
    cotwo: u16,
    temp: f32,
    humdt: f32,
}

#[async_std::main]
async fn main() {
    // env_logger::init();

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf: [u8; 4096] = [0; 4096];
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let mut socket = TcpStream::as_raw_fd(&stream);

    let mut config = ClientConfig::new(
        rust_mqtt::client::client_config::MqttVersion::MQTTv5,
        CountingRng(20000),
    );
    config.add_max_subscribe_qos(rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1);
    config.add_client_id("client");
    // config.add_username(USERNAME);
    // config.add_password(PASSWORD);
    config.max_packet_size = 100;
    let mut recv_buffer = [0; 80];
    let mut write_buffer = [0; 80];

    let mut client =
        MqttClient::<_, 5, _>::new(socket, &mut write_buffer, 80, &mut recv_buffer, 80, config);

    client.connect_to_broker().await.unwrap();

    loop {
        client
            .send_message(
                "hello",
                b"hello2",
                rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS0,
                true,
            )
            .await
            .unwrap();
        task::sleep(Duration::from_millis(500)).await;
    }
}
