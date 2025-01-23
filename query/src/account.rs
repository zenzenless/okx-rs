use std::{process::Output, vec};

use account::apis::{
    account_api::{self, ApiV5AccountConfigGetError},
    configuration::NewConf,
};
use finance::apis::finance_apis;
use fundings::apis::fundings_api::{self, ApiV5AssetBalancesGetError};

pub struct AuthInfo {
    api_key: String,
    pass_pahrase: String,
    sercrekey: String,
    base_url: String,
}

impl AuthInfo {
    pub fn new(api_key: String, pass_pahrase: String, sercrekey: String, base_url: String) -> Self {
        AuthInfo {
            api_key,
            pass_pahrase,
            sercrekey,
            base_url,
        }
    }
    pub fn to_conf<T: NewConf>(&self) -> T {
        T::new(
            self.base_url.clone(),
            self.api_key.clone(),
            self.sercrekey.clone(),
            self.pass_pahrase.clone(),
        )
    }
}

pub fn query_funding_asset(auth: &AuthInfo) {
    let assets = funding_account_assets(auth);
    match assets {
        Ok(assets) => {
            for i in assets {
                println!("funding {:?}: eqUsd:{:?} availBal{:?} fronzen:{:?}", i.ccy,i.eq_usd, i.availBal, i.fronzen_bal);
            }
        }
        Err(e) => {
            println!("Error fetching account balance: {:?}", e);
        }
    }
}
fn funding_account_assets(
    auth_info: &AuthInfo,
) -> Result<Vec<FundingAsset>, fundings::apis::Error<ApiV5AssetBalancesGetError>> {
    let conf = auth_info.to_conf::<fundings::apis::configuration::Configuration>();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async { fundings_api::api_v5_asset_balances_get(&conf, None).await });
    // Execute account query logic here
    match res {
        Ok(response) => {
            println!("API call successful!");
            let my_asset = response["data"].as_array().unwrap();
            let mut assets = vec![];
            for i in my_asset {
               
                assets.push(FundingAsset {
                    availBal: i["availBal"].as_str().and_then(|s|Some(s.parse().unwrap())),
                    ccy: i["ccy"].as_str().unwrap_or("").to_string(),
                    eq_usd: i["eqUsd"].as_str().and_then(|f|Some(f.parse().unwrap())),
                    fronzen_bal: i["frozenBal"].as_str().and_then(|f|Some(f.parse().unwrap())),
                });
            }
            Ok(assets)
        }
        Err(e) => {
            println!("API call failed!");
            println!("Error fetching account balance: {:?}", e);
            Err(e)
        }
    }
}
pub struct FundingAsset {
    pub ccy: String,
    pub eq_usd: Option<f64>,
    pub fronzen_bal: Option<f64>,
    pub availBal: Option<f64>,
}

pub fn query_trade_account_asset(auth: &AuthInfo) {
    let res = get_trade_account_asset(auth);
    match res {
        Ok(assets) => {
            for i in assets {
                println!("trading {:?}:eqUsd{:?} avaliBal:{:?} fronzen:{:?} stg:{:?}", i.ccy, i.eq_usd,i.availBal, i.fronzen_bal,i.stgyEq);
            }
        }
        Err(e) => {
            println!("Error fetching account balance: {:?}", e);
        }
    }
}
pub fn get_trade_account_asset(
    auth: &AuthInfo,
) -> Result<Vec<TradeAsset>, account::apis::Error<account_api::ApiV5AccountBalanceGetError>> {
    let conf: account::apis::configuration::Configuration = auth.to_conf();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async { account_api::api_v5_account_balance_get(&conf, None).await });
    match res {
        Ok(response) => {
            let mut asset = vec![];
            println!("API call successful!");
            if let Some(data) = response["data"].as_array() {
                for i in data {
                    let detail = i["details"].as_array().unwrap();
                    for j in detail {
                
                        asset.push(TradeAsset {
                            ccy: j["ccy"].as_str().unwrap_or("").to_string(),
                            eq_usd: j["eqUsd"].as_str().and_then(|s|Some(s.parse().unwrap())),
                            fronzen_bal: j["frozenBal"].as_str().and_then(|s|Some(s.parse().unwrap())),
                            stgyEq: j["stgyEq"].as_str().and_then(|s|Some(s.parse().unwrap())),
                            availBal: j["availBal"].as_str().and_then(|s|Some(s.parse().unwrap())),
                        });
                    }
                }
            }
            Ok(asset)
        }
        Err(e) => {
            println!("API call failed!");
            println!("Error fetching account balance: {:?}", e);
            Err(e)
        }
    }
}
pub struct TradeAsset {
    pub ccy: String,
    pub eq_usd: Option<f64>,
    pub fronzen_bal: Option<f64>,
    pub stgyEq: Option<f64>,
    pub availBal: Option<f64>,

}
pub fn query_finance_balance(auth: &AuthInfo) {
    let asset = get_finance_balance(auth);
    match asset {
        Ok(asset) => {
            for i in asset {
                println!(
                    "finance {:?}: earnings:{:?} redempt_amt:{:?} rate:{:?} amt:{:?} loan_amt:{:?} pending_amt:{:?}",
                    i.ccy, i.earnings, i.redempt_amt, i.rate, i.amt,i.loan_amt,i.pending_amt
                );
            }
        }
        Err(e) => {
            println!("Error fetching account balance: {:?}", e);
        }
    }
}    

pub fn get_finance_balance(auth: &AuthInfo)->Result<Vec<finance::models::savings_balance::SavingsBalanceData>, anyhow::Error>{
    let conf= auth.to_conf::<finance::apis::configuration::Configuration>();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async { finance_apis::get_savings_balance(&conf, None).await });
    match res {
        Ok(response) => {
            println!("API call successful!");
            if let Some( etr) = response.entity {
                return Ok(etr.data);
            }
            Err(anyhow::anyhow!(""))
        }
        Err(e) => {
            println!("API call failed!");
            println!("Error fetching account balance: {:?}", e);
            Err(e)
        }
    }
}


// pub fn query_free_asset<T>(api_key: T, passphrase: T, secretkey: T)
// where
//     T: Into<String>,
// {
// }
