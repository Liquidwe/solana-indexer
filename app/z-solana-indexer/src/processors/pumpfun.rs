use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult,
    instruction::InstructionProcessorInputType,
    metrics::MetricsCollection,
    processor::Processor,
};
use carbon_pumpfun_decoder::instructions::PumpfunInstruction;
use std::sync::Arc;

pub struct PumpfunInstructionProcessor;

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    type InputType = InstructionProcessorInputType<PumpfunInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let pumpfun_instruction: PumpfunInstruction = data.1.data;

        match pumpfun_instruction {
            PumpfunInstruction::CreateEvent(create_event) => {
                // println!("\nNew token created: {:#?}", create_event);
            }
            PumpfunInstruction::TradeEvent(trade_event) => {
                // println!("\nBig trade occured: {:#?}", trade_event);
            }
            PumpfunInstruction::CompleteEvent(complete_event) => {
                // println!("\nBonded: {:#?}", complete_event);
            }
            _ => {}
        };

        Ok(())
    }
}