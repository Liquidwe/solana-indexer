mod processors;
mod constants;
mod sinks;

use processors::{
    RaydiumAmmV4InstructionProcessor,
    PumpfunInstructionProcessor,
    MeteoraInstructionProcessor,
    RaydiumClmmInstructionProcessor,
};
use constants::{RAYDIUM_AMM_V4_PROGRAM_ID, PUMPFUN_PROGRAM_ID, METEORA_PROGRAM_ID, RAYDIUM_CLMM_PROGRAM_ID};
use carbon_raydium_amm_v4_decoder::RaydiumAmmV4Decoder;
use carbon_meteora_dlmm_decoder::MeteoraDlmmDecoder;
use carbon_pumpfun_decoder::PumpfunDecoder;
use carbon_raydium_clmm_decoder::RaydiumClmmDecoder;
use carbon_core::error::CarbonResult;
use helius::types::{
    Cluster, RpcTransactionsConfig, TransactionCommitment, TransactionDetails,
    TransactionSubscribeFilter, TransactionSubscribeOptions, UiEnhancedTransactionEncoding,
};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::RwLock;
use sinks::clickhouse::ClickhouseSink;

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
                        METEORA_PROGRAM_ID.to_string().clone(),
                        RAYDIUM_CLMM_PROGRAM_ID.to_string().clone(),
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
        // .instruction(RaydiumClmmDecoder, RaydiumClmmInstructionProcessor)
        // .instruction(MeteoraDlmmDecoder, MeteoraInstructionProcessor)
        .sink(Arc::new(ClickhouseSink::new(std::env::var("CLICKHOUSE_URL").unwrap())))
        .build()?
        .run()
        .await?;

    Ok(())
}