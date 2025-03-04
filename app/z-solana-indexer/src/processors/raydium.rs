use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult,
    metrics::MetricsCollection,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    processor::Processor,
};
use carbon_raydium_amm_v4_decoder::instructions::RaydiumAmmV4Instruction;
use std::sync::Arc;

pub struct RaydiumAmmV4InstructionProcessor;

#[async_trait]
impl Processor for RaydiumAmmV4InstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<RaydiumAmmV4Instruction>,
        Vec<NestedInstruction>,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;
        let accounts = instruction.accounts;

        match instruction.data {
            RaydiumAmmV4Instruction::Initialize2(init_pool) => {
            }
            RaydiumAmmV4Instruction::SwapBaseIn(swap) => {
                println!(
                    "\nsignature: {:#?}\nSwap: {:#?}",
                    signature, swap
                );
            }
            RaydiumAmmV4Instruction::SwapBaseOut(swap) => {
            }
            _ => {}
        };

        Ok(())
    }
}