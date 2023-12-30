use chrono::Utc;
use log::{debug, error};
use rumqttc::Packet;
use uuid::Uuid;

use crate::{db::DBClient, models::NewRecord};

use self::mqtt_instancer::MqttDaemon;

pub mod mqtt_instancer {
    use rumqttc::{AsyncClient, EventLoop, MqttOptions};
    use std::time::Duration;

    pub struct MqttDaemon {}

    impl MqttDaemon {
        pub fn new_daemon(id: &str) -> (AsyncClient, EventLoop) {
            let mut mqtt_options = MqttOptions::new(id, "localhost", 1883);
            mqtt_options.set_keep_alive(Duration::from_secs(30));
            AsyncClient::new(mqtt_options, 1024)
        }
    }
}

#[actix_web::main]
pub async fn mqtt_listening(db: DBClient) {
    // !important: enough randomness to avoid being kicked by a malicious client with the same id
    let (mut client, mut eventloop) =
        MqttDaemon::new_daemon(("MQTT_DAEMON".to_string() + &Uuid::new_v4().to_string()).as_str());
    client
        .subscribe("#", rumqttc::QoS::ExactlyOnce)
        .await
        .expect("Subscribe failed!");
    'eventloop: loop {
        let notification = eventloop.poll().await;
        let notification = match notification {
            Err(_) => {
                // May be kicked...
                let (new_client, new_eventloop) = MqttDaemon::new_daemon(
                    ("MQTT_DAEMON".to_string() + &Uuid::new_v4().to_string()).as_str(),
                );
                client = new_client;
                eventloop = new_eventloop;
                client
                    .subscribe("#", rumqttc::QoS::ExactlyOnce)
                    .await
                    .expect("Subscribe failed!");
                continue 'eventloop;
            }
            Ok(event) => event,
        };

        if let rumqttc::Event::Incoming(Packet::Publish(published)) = notification {
            debug!(
                "got topic={} payload={:?}",
                published.topic, published.payload
            );
            let device = db.get_device_by_topic(&published.topic).await;
            let device = match device {
                Ok(device) => device,
                Err(e) => {
                    error!(
                        "Unable to find the device registered for the topic: {:?}",
                        e
                    );
                    continue 'eventloop;
                }
            };
            let res = db
                .add_device_records(&NewRecord {
                    did: device.id,
                    payload: &published.payload,
                    timestamp: &Utc::now().naive_utc(),
                })
                .await;
            if let Err(e) = res {
                error!("Insert record failed: {:?}", e);
                continue 'eventloop;
            } else {
                crate::handlers::SYSINFO_CACHE
                    .buffer
                    .write()
                    .await
                    .record_count += 1;
                continue 'eventloop;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mqtt_instancer::MqttDaemon;
    use std::thread;
    #[tokio::test]
    async fn send_test() {
        let (client, mut eventloop) = MqttDaemon::new_daemon("MQTT_DAEMON");
        client
            .publish(
                "any/hello",
                rumqttc::QoS::AtMostOnce,
                false,
                vec![1, 2, 3, 4, 5],
            )
            .await
            .unwrap();
        for _ in 0..2 {
            let _notification = eventloop.poll().await.unwrap(); // Must poll to drive it
        }
    }
    #[tokio::main]
    async fn receive() {
        use rumqttc::QoS;

        let (client, mut eventloop) = MqttDaemon::new_daemon("receiver");
        client.subscribe("#", QoS::AtMostOnce).await.unwrap();

        loop {
            let notification = eventloop.poll().await.unwrap();
            println!("!Received = {:?}", notification);
        }
    }
    #[tokio::main]
    async fn send() {
        use rumqttc::QoS;
        use std::time::Duration;
        use tokio::{task, time};

        time::sleep(Duration::from_millis(1000)).await;
        let (client, mut eventloop) = MqttDaemon::new_daemon("sender");
        let cloned = client.clone();
        task::spawn(async move {
            for i in 0..10 {
                client
                    .publish(
                        "hello/thread11111",
                        QoS::AtLeastOnce,
                        false,
                        vec![i; i as usize],
                    )
                    .await
                    .unwrap();
                time::sleep(Duration::from_millis(100)).await;
            }
        });
        task::spawn(async move {
            for i in 0..10 {
                cloned
                    .publish("t2/t2", QoS::AtLeastOnce, false, vec![i; i as usize])
                    .await
                    .unwrap();
                time::sleep(Duration::from_millis(100)).await;
            }
        });
        loop {
            let notification = eventloop.poll().await.unwrap();
            println!("Sender: {:?}", notification);
        }
    }
    #[test]
    fn test() {
        // Receiver
        let h1 = thread::spawn(move || {
            println!("start1");
            receive()
        });

        // Sender
        let h2 = thread::spawn(move || {
            println!("start1");
            send()
        });

        h1.join().unwrap();
        h2.join().unwrap();
    }
}
