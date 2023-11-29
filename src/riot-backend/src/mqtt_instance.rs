pub mod mqtt_instancer {
    use rumqttc::{Client, Connection, MqttOptions};
    use std::time::Duration;

    pub struct MqttDaemon {}

    impl MqttDaemon {
        pub fn new_daemon() -> (Client, Connection) {
            let mut mqtt_options = MqttOptions::new("dynamic_server", "localhost", 1883);
            mqtt_options.set_keep_alive(Duration::from_secs(5));
            Client::new(mqtt_options, 1024)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mqtt_instancer::MqttDaemon;
    use rumqttc::Event::Incoming;
    use rumqttc::{Packet, QoS};
    use std::{thread, time::Duration};

    #[tokio::test]
    async fn test() {
        let (client, mut connection) = MqttDaemon::new_daemon();

        // MQTT Server thread
        thread::spawn(move || {
            for notification in connection.iter() {
                if let Incoming(Packet::Publish(published)) = notification.unwrap() {
                    println!(
                        "got topic={} payload={:?}",
                        published.topic, published.payload
                    )
                }
            }
        });

        // Virtual clients
        let mut cloned_client = client.clone();
        let h1 = thread::spawn(move || {
            for i in 0..100 {
                let payload = format!("publish {}", i);
                let sleep_time = Duration::from_secs(1);
                thread::sleep(sleep_time);
                cloned_client
                    .clone()
                    .publish("hello/world", QoS::AtLeastOnce, false, payload.clone())
                    .unwrap();
                cloned_client
                    .publish("aaa/aaa", QoS::AtLeastOnce, false, payload)
                    .unwrap();
            }
        });

        let mut cloned_client = client.clone();
        let h2 = thread::spawn(move || {
            for i in 0..100 {
                let sleep_time = Duration::from_secs(1);
                thread::sleep(sleep_time);
                if i % 10 == 0 {
                    if (i / 10) % 2 == 0 {
                        cloned_client
                            .subscribe("hello/world", QoS::AtLeastOnce)
                            .unwrap();
                    } else {
                        cloned_client.unsubscribe("hello/world").unwrap();
                    }
                    if (i / 10) % 2 == 1 {
                        cloned_client
                            .subscribe("aaa/aaa", QoS::ExactlyOnce)
                            .unwrap();
                    } else {
                        cloned_client.unsubscribe("aaa/aaa").unwrap();
                    }
                }
            }
        });
        h1.join().unwrap();
        h2.join().unwrap();
    }
}
