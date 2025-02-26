use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde_json::to_string;
use std::time::Duration;

use carbon_core::error::CarbonResult;
use crate::events::DexEvent;

pub struct KafkaSink {
    producer: FutureProducer,
    topic: String,
}

impl KafkaSink {
    pub fn new(brokers: &str, topic: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Failed to create Kafka producer");

        Self {
            producer,
            topic: topic.to_string(),
        }
    }

    pub async fn send_event(&self, event: DexEvent) -> CarbonResult<()> {
        let payload = to_string(&event).unwrap();
        
        self.producer
            .send(
                FutureRecord::to(&self.topic)
                    .payload(&payload)
                    .key("aaa"),
                Duration::from_secs(5),
            )
            .await
            .map_err(|e| carbon_core::error::Error::Custom(e.0.to_string()))?;

        log::info!("Sent event to Kafka: {}", payload);
        Ok(())
    }
}
