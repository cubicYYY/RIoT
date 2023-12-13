pub mod mqtt_instancer {
    use rumqttc::{AsyncClient, EventLoop, MqttOptions};
    use std::time::Duration;

    pub struct MqttDaemon {}

    impl MqttDaemon {
        pub fn new_daemon() -> (AsyncClient, EventLoop) {
            let mut mqtt_options = MqttOptions::new("dynamic_server", "localhost", 1883);
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
        let (client, _eventloop) = MqttDaemon::new_daemon();
        client
            .publish(
                "any/hello",
                rumqttc::QoS::AtMostOnce,
                false,
                vec![1, 2, 3, 4, 5],
            )
            .await
            .unwrap();
    }

    #[test]
    fn test() {
        // Receiver
        let h1 = thread::spawn(move || {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    // = tokio::main
                    use rumqttc::QoS;
                    use std::time::Duration;
                    use tokio::{task, time};

                    let (client, mut eventloop) = MqttDaemon::new_daemon();
                    client
                        .subscribe("hello/thread11111", QoS::AtMostOnce)
                        .await
                        .unwrap();
                    client.subscribe("t2/t2", QoS::AtMostOnce).await.unwrap();

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

                    loop {
                        let notification = eventloop.poll().await.unwrap();
                        println!("Received = {:?}", notification);
                    }
                });
        });

        // Sender
        let h2 = thread::spawn(move || {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    // = tokio::main
                    use rumqttc::QoS;
                    use std::time::Duration;
                    use tokio::{task, time};

                    let (client, mut eventloop) = MqttDaemon::new_daemon();
                    task::spawn(async move {
                        for i in 0..10 {
                            client
                                .publish("t2/t2", QoS::AtLeastOnce, false, vec![i; i as usize])
                                .await
                                .unwrap();
                            time::sleep(Duration::from_millis(100)).await;
                        }
                    });

                    loop {
                        let notification = eventloop.poll().await.unwrap();
                        println!("Received = {:?}", notification);
                    }
                });
        });

        h1.join().unwrap();
        h2.join().unwrap();
    }
}
