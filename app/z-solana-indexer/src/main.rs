use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult, instruction::InstructionProcessorInputType, metrics::MetricsCollection,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    processor::Processor,
};
use carbon_raydium_amm_v4_decoder::{
    instructions::RaydiumAmmV4Instruction, RaydiumAmmV4Decoder,
};
use carbon_meteora_dlmm_decoder::{instructions::MeteoraDlmmInstruction, MeteoraDlmmDecoder};
use carbon_pumpfun_decoder::{instructions::PumpfunInstruction, PumpfunDecoder};
pub const RAYDIUM_AMM_V4_PROGRAM_ID: Pubkey =
    pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
use helius::types::{
    Cluster, RpcTransactionsConfig, TransactionCommitment, TransactionDetails,
    TransactionSubscribeFilter, TransactionSubscribeOptions, UiEnhancedTransactionEncoding,
};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::RwLock;

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
                println!(
                    "\nsignature: {:#?}\nInitialize: {:#?}\nAccounts: {:#?}",
                    signature, init_pool, accounts
                );
            }
            RaydiumAmmV4Instruction::SwapBaseIn(swap) => {
                println!(
                    "\nsignature: {:#?}\nSwap: {:#?}",
                    signature, swap
                );
            }
            RaydiumAmmV4Instruction::SwapBaseOut(swap) => {
                println!(
                    "\nsignature: {:#?}\nSwap: {:#?}",
                    signature, swap
                );
            }
            _ => {}
        };

        Ok(())
    }
}

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
                println!("\nNew token created: {:#?}", create_event);
            }
            PumpfunInstruction::TradeEvent(trade_event) => {
                println!("\nBig trade occured: {:#?}", trade_event);
            }
            PumpfunInstruction::CompleteEvent(complete_event) => {
                println!("\nBonded: {:#?}", complete_event);
            }
            _ => {}
        };

        Ok(())
    }
}

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
            MeteoraDlmmInstruction::Swap(_swap) => {
                println!("\nSignature: {:#?}", _instruction_metadata.transaction_metadata.signature);
                println!("\nMeteora Swap: {:#?}", _swap);
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

pub const PUMPFUN_PROGRAM_ID: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let helius_websocket = carbon_helius_atlas_ws_datasource::HeliusWebsocket::new(
        std::env::var("API_KEY").unwrap(),
        carbon_helius_atlas_ws_datasource::Filters {
            accounts: vec![],
            transactions: Some(RpcTransactionsConfig {
                filter: TransactionSubscribeFilter {
                    account_include: Some(vec![
                        PUMPFUN_PROGRAM_ID.to_string().clone(),
                        RAYDIUM_AMM_V4_PROGRAM_ID.to_string().clone(),
                    ]),
                    account_exclude: None,
                    account_required: None,
                    vote: None,
                    failed: None,
                    signature: None,
                },
                options: TransactionSubscribeOptions {
                    commitment: Some(TransactionCommitment::Confirmed),
                    encoding: Some(UiEnhancedTransactionEncoding::Base64),
                    transaction_details: Some(TransactionDetails::Full),
                    show_rewards: None,
                    max_supported_transaction_version: Some(0),
                },
            }),
        },
        Arc::new(RwLock::new(HashSet::new())),
        Cluster::MainnetBeta,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(helius_websocket)
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .instruction(RaydiumAmmV4Decoder, RaydiumAmmV4InstructionProcessor)
        .instruction(MeteoraDlmmDecoder, MeteoraInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}