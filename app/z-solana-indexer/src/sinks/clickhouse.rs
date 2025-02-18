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

#[async_trait]
impl Sink for ClickhouseSink {
    async fn initialize(&self) -> CarbonResult<()> {
        Ok(())
    }

    async fn flush(&self) -> CarbonResult<()> {
        self.processed_count.store(self.get_processed_count()+1, std::sync::atomic::Ordering::Relaxed);
        println!("flushing clickhouse sink, processed {} records", self.get_processed_count());
        Ok(())
    }

    async fn shutdown(&self) -> CarbonResult<()> {
        Ok(())
    }

    // async fn process(&self, data: Vec<Row>) -> CarbonResult<()> {
    //     self.client
    //         .insert("")
    //         .data(data)
    //         .await?;
        
    //     Ok(())
    // }
}