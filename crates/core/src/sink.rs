use crate::error::CarbonResult;
use async_trait::async_trait;

/// Trait for implementing data sinks in the pipeline.
///
/// A Sink is responsible for persisting or forwarding processed data to external systems.
/// This could be databases, message queues, or other storage/transmission mechanisms.
#[async_trait]
pub trait Sink: Send + Sync {
    /// Initializes the sink, establishing connections or preparing resources.
    async fn initialize(&self) -> CarbonResult<()>;

    /// Flushes any buffered data to ensure it's persisted.
    async fn flush(&self) -> CarbonResult<()>;

    /// Shuts down the sink, cleaning up resources and connections.
    async fn shutdown(&self) -> CarbonResult<()>;
}