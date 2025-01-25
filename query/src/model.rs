// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::FinanceAsset;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: FinanceAsset = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAsset {
    pub earnings: String,
    pub redempt_amt: String,
    pub rate: String,
    pub ccy: String,
    pub amt: String,
    pub loan_amt: String,
    pub pending_amt: String,
}

#[derive(Serialize, Deserialize)]
pub struct CoinsInfo {
    pub id: String,

    pub symbol: String,

    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinPrice {
    pub inst_type: String,

    pub inst_id: String,

    pub mark_px: String,

    ts: String,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexTicker {
  inst_id: String,

  pub idx_px: String,

  #[serde(rename = "high24h")]
  high24_h: String,

  sod_utc0: String,

  #[serde(rename = "open24h")]
  open24_h: String,

  #[serde(rename = "low24h")]
  low24_h: String,

  sod_utc8: String,

  ts: String,
}
