use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult,
    instruction::InstructionProcessorInputType,
    metrics::MetricsCollection,
    processor::Processor,
};
use carbon_pumpfun_decoder::instructions::PumpfunInstruction;
use chrono::Utc;
use std::sync::Arc;
use crate::sinks::kafka::KafkaSink;

use crate::events::{DexEvent, TradeEvent, CreatePoolEvent};

pub struct PumpfunInstructionProcessor {
    sink: Arc<KafkaSink>,
}

impl PumpfunInstructionProcessor {
    pub fn new(sink: Arc<KafkaSink>) -> Self {
        Self { sink }
    }
}

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    type InputType = InstructionProcessorInputType<PumpfunInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, instruction, _nested) = data;
        let pumpfun_instruction = instruction.data;

        match pumpfun_instruction {
            PumpfunInstruction::TradeEvent(trade_event) => {
                let event = DexEvent::Trade(TradeEvent {
                    program: "pumpfun".to_string(),
                    timestamp: trade_event.timestamp,
                    tx_hash: metadata.transaction_metadata.signature.to_string(),
                    mint: trade_event.mint.to_string(),
                });

                self.sink.send_event(event).await?;

                metrics.increment_counter("trade_events_processed", 1).await?;
            }
            PumpfunInstruction::CreateEvent(create_event) => {
                let event = DexEvent::CreatePool(CreatePoolEvent {
                    program: "pumpfun".to_string(),
                    timestamp: Utc::now().timestamp(),
                    signature: metadata.transaction_metadata.signature.to_string(),
                });


                metrics.increment_counter("create_pool_events_processed", 1).await?;
            }
            _ => {}
        };

        Ok(())
    }
}