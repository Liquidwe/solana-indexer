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
use crate::events::{SolanaEvent, TradeEvent, CreatePoolEvent};
use crate::utils::pumpfun_pda::get_bonding_curve_by_mint;

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
                let (bonding_curve, _) = get_bonding_curve_by_mint(&trade_event.mint.to_string())
                    .map_err(|e| carbon_core::error::Error::Custom(e.to_string()))?;
                
                let event = SolanaEvent::Trade(TradeEvent {
                    dex_type: "pumpfun".to_string(),
                    event_type: "trade".to_string(),
                    event_type_id: 1,
                    tx_hash: metadata.transaction_metadata.signature.to_string(),
                    slot_number: metadata.transaction_metadata.slot,
                    tx_index: 0,
                    event_index: 0,
                    pool_address: bonding_curve.to_string(),
                    base_address: "So11111111111111111111111111111111111111112".to_string(),
                    quote_address: "".to_string(),
                    base_amount_str: "".to_string(),
                    quote_amount_str: "".to_string(),
                    base_amount: 0,
                    quote_amount: 0,
                    maker_address: "".to_string(),
                    maker_tags: "".to_string(),
                    event_time: Utc::now().timestamp(),
                    virtual_sol_reserves: 0,
                    virtual_token_reserves: 0,
                    real_sol_reserves: 0,
                    real_token_reserves: 0,
                    is_buy: 0,
                    amount_usd: 0.0,
                    price_usd: 0.0,
                    price_quote: 0.0,
                });

                println!("event: {:?}", event);

                self.sink.send_event(event).await?;

                metrics.increment_counter("trade_events_processed", 1).await?;
            }
            
            PumpfunInstruction::CreateEvent(create_event) => {
                let event = SolanaEvent::CreatePool(CreatePoolEvent {
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