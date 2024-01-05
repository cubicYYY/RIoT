use chrono::Utc;
use log::{debug, error};
use rumqttc::Packet;
use uuid::Uuid;

use crate::{db::DBClient, models::NewRecord};

use self::mqtt_instancer::MqttDaemon;
/// MQTT util class
pub mod mqtt_instancer {
    use rumqttc::{AsyncClient, EventLoop, MqttOptions};
    use std::time::Duration;

    pub struct MqttDaemon {}

    /// Return a new daemon to be later invoked in a async runtime (e.g. tokio)
    impl MqttDaemon {
        pub fn new_daemon(id: &str, host: &str, port: u16) -> (AsyncClient, EventLoop) {
            let mut mqtt_options = MqttOptions::new(id, host, port);
            mqtt_options.set_keep_alive(Duration::from_secs(30));
            AsyncClient::new(mqtt_options, 1024)
        }
    }
}

#[actix_web::main]
/// MQTT Listening daemon
pub async fn mqtt_listening(db: DBClient, host: &str, port: u16) {
    // !important: enough randomness to avoid being kicked by a malicious client with the same id
    let (mut client, mut eventloop) = MqttDaemon::new_daemon(
        ("MQTT_DAEMON".to_string() + &Uuid::new_v4().to_string()).as_str(),
        host,
        port,
    );
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
                    host,
                    port,
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
            //topic must be start with user's api key
            let mut try_split = published.topic.split('/');
            let api_key = match try_split.next() {
                Some(api_key) => api_key,
                None => {
                    error!("Cannot find api key in topic: {:?}", published.topic);
                    continue 'eventloop;
                }
            };
            let user = match db.get_user_by_api_key(api_key).await {
                Ok(user) => user,
                Err(_) => {
                    error!("Unable to find the user with ApiKey= {:?}", api_key);
                    continue 'eventloop;
                }
            };
            let topic = &published.topic[(api_key.len() + 1)..]; // with api key stripped
            let device = db.get_device_by_topic(topic).await;
            let device = match device {
                Ok(device) => {
                    if device.uid != user.id {
                        error!("Device not owned by the user");
                        continue 'eventloop;
                    }
                    device
                }
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
    /// MQTT Message send test
    async fn send_test() {
        let (client, mut eventloop) = MqttDaemon::new_daemon("MQTT_DAEMON", "localhost", 1883);
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

        let (client, mut eventloop) = MqttDaemon::new_daemon("receiver", "localhost", 1883);
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
        let (client, mut eventloop) = MqttDaemon::new_daemon("sender", "localhost", 1883);
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
