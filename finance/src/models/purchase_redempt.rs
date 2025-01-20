use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseRedemptRequest {
    pub ccy: String,
    pub amt: String,
    pub side: String,
    pub rate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseRedemptResponse {
    pub code: String,
    pub msg: String,
    pub data: Vec<PurchaseRedemptData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseRedemptData {
    pub ccy: String,
    pub amt: String,
    pub side: String,
    pub rate: String,
}