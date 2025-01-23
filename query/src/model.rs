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

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAsset {
  pub  earnings: String,
  pub  redempt_amt: String,
  pub  rate: String,
  pub  ccy: String,
  pub  amt: String,
  pub  loan_amt: String,
  pub  pending_amt: String,
}
