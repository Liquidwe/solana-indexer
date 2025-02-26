use async_trait::async_trait;
use carbon_core::{error::CarbonResult, sink::Sink};
use clickhouse::{Client, Row};

pub struct ClickhouseSink {
    client: Client,
    processed_count: std::sync::atomic::AtomicU64,
}

impl ClickhouseSink {
    pub fn new(connection_string: String) -> Self {
        let client = Client::default()
            .with_url(connection_string);
        println!("clickhouse sink created");
        Self { 
            client,
            processed_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn get_processed_count(&self) -> u64 {
        self.processed_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}