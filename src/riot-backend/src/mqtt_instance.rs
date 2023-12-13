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
