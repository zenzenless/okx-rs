use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingsBalanceRequest {
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingsBalanceResponse {
    pub code: String,
    pub msg: String,
    pub data: Vec<SavingsBalanceData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingsBalanceData {
    pub earnings: String,
    #[serde(rename = "redemptAmt")]
    pub redempt_amt: String,
    pub rate: String,
    pub ccy: String,
    pub amt: String,
    #[serde(rename = "loanAmt")]
    pub loan_amt: String,
    #[serde(rename = "pendingAmt")]
    pub pending_amt: String,
}