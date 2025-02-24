use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult,
    metrics::MetricsCollection,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    processor::Processor,
};
use carbon_meteora_dlmm_decoder::instructions::MeteoraDlmmInstruction;
use std::sync::Arc;

pub struct MeteoraInstructionProcessor;

#[async_trait]
impl Processor for MeteoraInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<MeteoraDlmmInstruction>,
        Vec<NestedInstruction>,
    );

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (_instruction_metadata, decoded_instruction, _nested_instructions) = data;

        match decoded_instruction.data {
            MeteoraDlmmInstruction::AddLiquidity(_add_liquidity) => {}
            MeteoraDlmmInstruction::RemoveLiquidity(_remove_liquidity) => {}
            MeteoraDlmmInstruction::SwapEvent(_swap) => {
                println!("signature: {:?}", _instruction_metadata.transaction_metadata.signature);
                println!("SwapEvent: {:?}", _swap);
            }
            MeteoraDlmmInstruction::ClaimReward(_claim_reward) => {}
            MeteoraDlmmInstruction::FundReward(_fund_reward) => {}
            MeteoraDlmmInstruction::InitializeReward(_initialize_reward) => {}
            MeteoraDlmmInstruction::UpdateRewardFunder(_update_reward_funder) => {}
            MeteoraDlmmInstruction::UpdateRewardDuration(_update_reward_duration) => {}
            MeteoraDlmmInstruction::ClaimFee(_claim_fee) => {}
            MeteoraDlmmInstruction::ClosePosition(_position_close) => {}
            MeteoraDlmmInstruction::LbPairCreateEvent(_lb_pair_create) => {}
            MeteoraDlmmInstruction::PositionCreateEvent(_position_create) => {}
            MeteoraDlmmInstruction::FeeParameterUpdateEvent(_fee_parameter_update) => {}
            MeteoraDlmmInstruction::IncreaseObservationEvent(_increase_observation) => {}
            MeteoraDlmmInstruction::WithdrawIneligibleReward(_withdraw_ineligible_reward) => {}
            MeteoraDlmmInstruction::UpdatePositionOperator(_update_position_operator) => {}
            MeteoraDlmmInstruction::UpdatePositionLockReleasePointEvent(
                _update_position_lock_release_point,
            ) => {}
            MeteoraDlmmInstruction::InitializeLbPair(_initialize_lb_pair) => {}
            MeteoraDlmmInstruction::GoToABin(_go_to_abin) => {}
            _ => {}
        };

        Ok(())
    }
}