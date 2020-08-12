use rumqttc::{MqttOptions, EventLoop, Request, QoS, Incoming, Key};
use std::{env, fs};
use std::time::{Duration, Instant};
use async_channel::Sender;
use rand::Rng;
use tokio::task;
use tokio::time;
use futures;

#[macro_use]
extern crate log;

#[tokio::main(core_threads = 4)]
async fn main() {

    let mut handles = vec![];
    handles.push(task::spawn(async move {
        start().await;
    }));
    futures::future::join_all(handles).await;

}

async fn start() {

    println!("COMES HERE");
    let ca_path = env::var("VI_BROKER_CA_PATH").unwrap();
    let client_cert = env::var("VI_CLIENT_CERT_PATH").unwrap();
    let client_key = env::var("VI_CLIENT_PRIVATE_KEY_PATH").unwrap();


    let mut mqtt_options = MqttOptions::new("test_client", "ingest.staging.ather.vimana.us", 443);
    mqtt_options.set_keep_alive(30);
    mqtt_options.set_inflight(200);
    mqtt_options.set_conn_timeout(60);

    // change this to KEY::RSA if Using RSA.
    mqtt_options.set_key_type(Key::ECC);

    let ca = fs::read(ca_path).unwrap();
    mqtt_options.set_ca(ca);

    let cert = fs::read(client_cert).unwrap();
    let key = fs::read(client_key).unwrap();
    mqtt_options.set_client_auth(cert, key);

    let mut eventloop = EventLoop::new(mqtt_options, 100).await;
    let requests_tx = eventloop.handle();

    let topic = format!("hello/test_client/world");
    let qos = QoS::AtLeastOnce;
    task::spawn(async move {
        requests(topic, 100, requests_tx, qos).await;
    });

    loop {
        let res = match eventloop.poll().await {
            Ok(v) => v,
            Err(e) => {
                error!("connection error = {:?}", e);
                continue;
            }
        };
        let (inc, _ouc) = res;
        // debug!("inc = {:?}", inc);
        match inc {
            Some(v) => {
                match v {
                    Incoming::PubAck(pkid) => {
                        println!("Received PubAck for {:?}", &pkid.pkid);
                        continue;
                    },
                    Incoming::SubAck(suback) => {
                        println!("Received SubAck for {:?}", &suback.pkid);
                        continue;
                    },
                    Incoming::Publish(publish) => {
                        println!("Received Publish for {:?}", &publish.pkid);
                        continue;
                    },
                    _ => {continue},
                }
            }
            None => {}
        }
    }
}

async fn requests(topic: String, count: usize, requests_tx: Sender<Request>, qos: QoS) {
    for i in 0..count {
        let mut payload = generate_payload(500);
        payload[0] = (i % 255) as u8;
        let publish = rumqttc::Publish::new(&topic, qos, payload);
        let publish = Request::Publish(publish);
        if let Err(_) = requests_tx.send(publish).await {
            break;
        }
    }
    time::delay_for(Duration::from_secs(5)).await;
}

fn generate_payload(payload_size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let payload: Vec<u8> = (0..payload_size).map(|_| rng.gen_range(0, 255)).collect();
    payload
}