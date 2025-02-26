use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum DexEvent {
    Trade(TradeEvent),
    CreatePool(CreatePoolEvent),
}

#[derive(Debug, Serialize)]
pub struct TradeEvent {
    pub program: String,
    pub timestamp: i64,
    pub tx_hash: String,
    pub mint: String,
}

#[derive(Debug, Serialize)]
pub struct CreatePoolEvent {
    pub program: String,
    pub timestamp: i64,
    pub signature: String,
} 