use std::time::Duration;

use crate::models::KafkaEvent;
use rdkafka::{
    config::ClientConfig,
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
    message::Message,
};
use tokio::sync::broadcast::Sender;

pub async fn run_consumer(topic: &str, tx: Sender<KafkaEvent>) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "kafka1:9092,kafka2:9092,kafka3:9092")
        .set("group.id", format!("streaming-api-{}", topic))
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "latest")
        .create()
        .expect("Kafka: failed to create consumer");

    consumer
        .subscribe(&[topic])
        .expect(&format!("Kafka: failed to subscribe to topic {}", topic));

    println!("Kafka consumer started");
    println!("Waiting for messages in topics...");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let payload = match msg.payload_view::<str>() {
                    Some(Ok(s)) => s.to_string(),
                    _ => continue,
                };

                let event = KafkaEvent {
                    topic: msg.topic().to_string(),
                    payload,
                };

                let _ = tx.send(event);
            }

            // это НОРМАЛЬНО при partitions=3
            Err(KafkaError::PartitionEOF(_)) => {}
            Err(KafkaError::MessageConsumption(_)) => {
                println!("Waiting for topic to be created...");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
            // любая другая ошибка — падение
            Err(e) => panic!("Kafka runtime error: {}", e),
        }
    }
}
