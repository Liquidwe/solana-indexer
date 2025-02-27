use serde::Serialize;

#[derive(Debug)]
pub enum SolanaEvent {
    Trade(TradeEvent),
    CreatePool(CreatePoolEvent),
}

#[derive(Debug, Serialize)]
pub struct TradeEvent {
    pub dex_type: String,
    pub event_type: String,
    pub event_type_id: i32,
    pub tx_hash: String,
    pub slot_number: u64,
    pub tx_index: i32,
    pub event_index: i32,
    pub pool_address: String,
    pub base_address: String,
    pub quote_address: String,
    pub base_amount_str: String,
    pub quote_amount_str: String,
    pub base_amount: i64,
    pub quote_amount: i64,
    pub maker_address: String,
    pub maker_tags: String,
    pub event_time: i64,
    pub virtual_sol_reserves: i64,
    pub virtual_token_reserves: i64,
    pub real_sol_reserves: i64,
    pub real_token_reserves: i64,
    pub is_buy: i32,
    pub amount_usd: f64,
    pub price_usd: f64,
    pub price_quote: f64,
}

#[derive(Debug, Serialize)]
pub struct CreatePoolEvent {
    pub program: String,
    pub timestamp: i64,
    pub signature: String,
} 