// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::BalanceResponse;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: BalanceResponse = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BalanceResponse {
   pub  code: String,

   pub  data: Vec<Datum>,

   pub  msg: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datum {
pub   adj_eq: String,
pub   borrow_froz: String,
pub   details: Vec<Detail>,
pub   imr: String,
pub   iso_eq: String,
pub   mgn_ratio: String,
pub   mmr: String,
pub   notional_usd: String,
pub   ord_froz: String,
pub   total_eq: String,
pub   u_time: String,
pub   upl: String,
}

#[derive(Debug,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
pub acc_avg_px: String,
pub avail_bal: String,
pub avail_eq: String,
pub borrow_froz: String,
pub cash_bal: String,
pub ccy: String,
pub cl_spot_in_use_amt: String,
pub cross_liab: String,
pub dis_eq: String,
pub eq: String,
pub eq_usd: String,
pub fixed_bal: String,
pub frozen_bal: String,
pub imr: String,
pub interest: String,
pub iso_eq: String,
pub iso_liab: String,
pub iso_upl: String,
pub liab: String,
pub max_loan: String,
pub max_spot_in_use: String,
pub mgn_ratio: String,
pub mmr: String,
pub notional_lever: String,
pub open_avg_px: String,
pub ord_frozen: String,
pub reward_bal: String,
pub smt_sync_eq: String,
pub spot_bal: String,
pub spot_copy_trading_eq: String,
pub spot_in_use_amt: String,
pub spot_iso_bal: String,
pub spot_upl: String,
pub spot_upl_ratio: String,
pub stgy_eq: String,
pub total_pnl: String,
pub total_pnl_ratio: String,
pub twap: String,
pub u_time: String,
pub upl: String,
pub upl_liab: String,
}
