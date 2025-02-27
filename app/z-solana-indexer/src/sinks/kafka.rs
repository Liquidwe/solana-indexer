use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde_json::to_string;
use std::time::Duration;

use carbon_core::error::CarbonResult;
use crate::events::SolanaEvent;

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

    pub async fn send_event(&self, event: SolanaEvent) -> CarbonResult<()> {
        let (payload, key) = match event {
            SolanaEvent::Trade(trade_event) => {
                (to_string(&trade_event).unwrap(), trade_event.tx_hash.clone())
            },
            SolanaEvent::CreatePool(create_event) => {
                (to_string(&create_event).unwrap(), create_event.signature.clone())
            },
        };
        
        self.producer
            .send(
                FutureRecord::to(&self.topic)
                    .payload(&payload)
                    .key(&key),
                Duration::from_secs(2),
            )
            .await
            .map_err(|e| carbon_core::error::Error::Custom(e.0.to_string()))?;

        log::info!("Sent event to Kafka: {}", payload);
        Ok(())
    }
}
