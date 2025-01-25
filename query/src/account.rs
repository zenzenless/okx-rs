use std::{collections::HashMap, process::Output, vec};

use account::apis::{
    account_api::{self, ApiV5AccountConfigGetError},
    configuration::NewConf,
};
use finance::apis::finance_apis;
use fundings::apis::fundings_api::{self, ApiV5AssetBalancesGetError};
use market::apis::market_data_api;
use public::apis::configuration;

use crate::{
    gecko,
    model::{self, CoinPrice, IndexTicker},
};

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
                println!(
                    "funding {:?}: eqUsd:{:?} availBal{:?} fronzen:{:?}",
                    i.ccy, i.eq_usd, i.availBal, i.fronzen_bal
                );
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
                    availBal: i["availBal"]
                        .as_str()
                        .and_then(|s| Some(s.parse().unwrap())),
                    ccy: i["ccy"].as_str().unwrap_or("").to_string(),
                    eq_usd: i["eqUsd"].as_str().and_then(|f| Some(f.parse().unwrap())),
                    fronzen_bal: i["frozenBal"]
                        .as_str()
                        .and_then(|f| Some(f.parse().unwrap())),
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
                println!(
                    "trading {:?}:eqUsd{:?} avaliBal:{:?} fronzen:{:?} stg:{:?}",
                    i.ccy, i.eq_usd, i.avail_bal, i.fronzen_bal, i.stgy_eq
                );
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
                            eq_usd: j["eqUsd"].as_str().and_then(|s| Some(s.parse().unwrap())),
                            fronzen_bal: j["frozenBal"]
                                .as_str()
                                .and_then(|s| Some(s.parse().unwrap())),
                            stgy_eq: j["stgyEq"].as_str().and_then(|s| Some(s.parse().unwrap())),
                            avail_bal: j["availBal"]
                                .as_str()
                                .and_then(|s| Some(s.parse().unwrap())),
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
    pub stgy_eq: Option<f64>,
    pub avail_bal: Option<f64>,
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

pub fn get_finance_balance(
    auth: &AuthInfo,
) -> Result<Vec<finance::models::savings_balance::SavingsBalanceData>, anyhow::Error> {
    let conf = auth.to_conf::<finance::apis::configuration::Configuration>();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async { finance_apis::get_savings_balance(&conf, None).await });
    match res {
        Ok(response) => {
            println!("API call successful!");
            if let Some(etr) = response.entity {
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

pub fn query_free_asset(auth: &AuthInfo) -> Result<(), anyhow::Error> {
    let funding_asset = funding_account_assets(auth)?;
    let trade_asset = get_trade_account_asset(auth)?;
    let finance_asset = get_finance_balance(auth)?;


    let mut my_coins = HashMap::new();
    for fa in funding_asset {
        my_coins.insert(
            fa.ccy.clone(),
            FreeUsedAsset {
                ccy: fa.ccy.clone(),
                coin_num: fa.availBal.unwrap_or(0.0),
                eq_usd: 0.0,
            },
        );
    }

    for ta in trade_asset {
        if let Some(coin) = my_coins.get_mut(&ta.ccy) {
            coin.coin_num += ta.avail_bal.unwrap_or(0.0);
        } else {
            my_coins.insert(
                ta.ccy.clone(),
                FreeUsedAsset {
                    ccy: ta.ccy.clone(),
                    coin_num: ta.avail_bal.unwrap_or(0.0),
                    eq_usd: 0.0,
                },
            );
        }
    }

    for fa in finance_asset {
        if let Some(coin) = my_coins.get_mut(&fa.ccy) {
            let n: f64 = fa.amt.parse()?;
            coin.coin_num += n;
        } else {
            my_coins.insert(
                fa.ccy.clone(),
                FreeUsedAsset {
                    ccy: fa.ccy.clone(),
                    coin_num: fa.amt.parse()?,
                    eq_usd: 0.0,
                },
            );
        }
    }
    let mut my_coins=my_coins.into_iter().filter(|(k, v)| v.coin_num > 0.0).collect::<HashMap<_, _>>();
    let price_map = get_all_coins_price()?;
    //caulate eq_usd
    for (k, v) in my_coins.iter_mut() {
        //get coin price
        let inst_id = format!("{}-USDT", k.to_ascii_uppercase());
        
        if let Some(ticker) = price_map.get(&inst_id) {
            if let Ok(price) = ticker.idx_px.parse::<f64>() {
                v.eq_usd = v.coin_num * price;
            } else {
                println!("Invalid price format for {}", inst_id);
            }
        } else {
            println!("No price found for {}", inst_id);
        }
    }
    let my_coins=my_coins.iter().filter(|(k, v)| v.eq_usd > 0.01).collect::<HashMap<_, _>>();
    println!("{:#?}", my_coins);
    Ok(())
}

#[derive(Debug)]
pub struct FreeUsedAsset {
    pub ccy: String,
    pub coin_num: f64,
    pub eq_usd: f64,
}

pub fn get_all_coins_price() -> Result<HashMap<String, IndexTicker>, anyhow::Error> {
    let mut conf = market::apis::configuration::Configuration::new();
    conf.base_path = "https://aws.okx.com".to_string();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let price = market_data_api::api_v5_market_index_tickers_get(&conf, None, Some("USDT"));
    let res = rt.block_on(price)?;
    
    if res["code"] != "0" {
        return Err(anyhow::anyhow!(
            "failed to get price msg:{}",
            res["msg"]
        ));
    }
    
    let data = &res["data"];
    let tickers: Vec<IndexTicker> = serde_json::from_value(data.clone())?;
    
    let price_map: HashMap<String, IndexTicker> = tickers
        .into_iter()
        .map(|t| (t.inst_id.clone(), t))
        .collect();
        
    Ok(price_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coin_price() {
        let price = get_all_coins_price().unwrap();
        println!("{} price: {:#?}", "BTC_USD", price);
    }
}
