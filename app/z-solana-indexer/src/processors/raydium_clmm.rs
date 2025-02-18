use std::sync::Arc;
use async_trait::async_trait;
use carbon_core::error::CarbonResult;
use carbon_core::instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction};
use carbon_core::metrics::MetricsCollection;
use carbon_core::processor::Processor;
use carbon_raydium_clmm_decoder::instructions::RaydiumClmmInstruction;

pub struct RaydiumClmmInstructionProcessor;

#[async_trait]
impl Processor for RaydiumClmmInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<RaydiumClmmInstruction>,
        Vec<NestedInstruction>,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;

        match instruction.data {
            RaydiumClmmInstruction::CreateAmmConfig(create_amm_cfg) => {
            }
            RaydiumClmmInstruction::UpdateAmmConfig(update_amm_cfg) => {
            }
            RaydiumClmmInstruction::CreatePool(create_pool) => {
            }
            RaydiumClmmInstruction::UpdatePoolStatus(update_pool_status) => {
            }
            RaydiumClmmInstruction::CreateOperationAccount(create_opperation_acc) => {
            }
            RaydiumClmmInstruction::UpdateOperationAccount(update_opperation_acc) => {
            }
            RaydiumClmmInstruction::TransferRewardOwner(transfer_reward_owner) => {
            }
            RaydiumClmmInstruction::InitializeReward(init_reward) => {
            }
            RaydiumClmmInstruction::CollectRemainingRewards(collect_remaining_rewards) => {
            }
            RaydiumClmmInstruction::UpdateRewardInfos(update_reward_infos) => {
            }
            RaydiumClmmInstruction::SetRewardParams(set_reward_params) => {
            }
            RaydiumClmmInstruction::CollectProtocolFee(collect_protocol_fee) => {
            }
            RaydiumClmmInstruction::CollectFundFee(collect_fund_fee) => {
            }
            RaydiumClmmInstruction::OpenPosition(open_position) => {
            }
            RaydiumClmmInstruction::OpenPositionV2(open_position_v2) => {
            }
            RaydiumClmmInstruction::ClosePosition(close_position) => {
            }
            RaydiumClmmInstruction::IncreaseLiquidity(increase_liq) => {
            }
            RaydiumClmmInstruction::IncreaseLiquidityV2(increase_liq_v2) => {
            }
            RaydiumClmmInstruction::DecreaseLiquidity(decrease_liq) => {
            }
            RaydiumClmmInstruction::DecreaseLiquidityV2(decrease_liq_v2) => {
            }
            RaydiumClmmInstruction::Swap(swap) => {
            }
            RaydiumClmmInstruction::SwapV2(swap_v2) => {
            }
            RaydiumClmmInstruction::SwapRouterBaseIn(swap_base_in) => {
            }
            RaydiumClmmInstruction::ConfigChangeEvent(cfg_change_event) => {
            }
            RaydiumClmmInstruction::CreatePersonalPositionEvent(crete_personal_position) => {
            }
            RaydiumClmmInstruction::IncreaseLiquidityEvent(increase_liq_event) => {
            }
            RaydiumClmmInstruction::DecreaseLiquidityEvent(decrease_liq_event) => {
            }
            RaydiumClmmInstruction::LiquidityCalculateEvent(liq_calc_event) => {
            }
            RaydiumClmmInstruction::CollectPersonalFeeEvent(collect_personal_fee_event) => {
            }
            RaydiumClmmInstruction::UpdateRewardInfosEvent(update_reward_info_event) => {
            }
            RaydiumClmmInstruction::PoolCreatedEvent(pool_create_event) => {
            }
            RaydiumClmmInstruction::CollectProtocolFeeEvent(collect_protocol_fee_event) => {
            }
            RaydiumClmmInstruction::SwapEvent(swap_event) => {
            }
            RaydiumClmmInstruction::LiquidityChangeEvent(liq_change_event) => {
            }
        };

        Ok(())
    }
}