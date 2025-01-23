use std::{process::Output, vec};

use account::apis::{account_api::{self, ApiV5AccountConfigGetError}, configuration::NewConf};
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

pub fn query_account_asset<T>(api_key: T, passphrase: T, secretkey: T)
where
    T: Into<String>,
{
    let auth =AuthInfo{
        api_key:api_key.into(),
        pass_pahrase:passphrase.into(),
        sercrekey:secretkey.into(),
        base_url:"https://aws.okx.com".to_string(),
    };
    let assets=account_assets(&auth);
    match assets {
        Ok(assets) => {
            for i in assets {
                println!("{}:{}:fronzen:{}", i.ccy, i.availBal, i.fronzen_bal);
            }
        },
        Err(e) => {
            println!("Error fetching account balance: {:?}", e);
        }
    }
}
pub struct asset{
    pub ccy:String,
    pub eq_usd:i64,
    pub fronzen_bal:i64,
    pub availBal:i64,
}
fn account_assets(auth_info: &AuthInfo)->Result<Vec<asset>,fundings::apis::Error<ApiV5AssetBalancesGetError>> {
    let conf = auth_info.to_conf::<fundings::apis::configuration::Configuration>();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async { fundings_api::api_v5_asset_balances_get(&conf, None).await });
    // Execute account query logic here
    match res {
        Ok(response) => {
            println!("API call successful!");
            let my_asset = response["data"].as_array().unwrap();
            let mut assets=vec![];
            for i in my_asset {
                println!("{}:{}:fronzen:{}", i["ccy"], i["availBal"], i["frozenBal"]);
                assets.push(asset{
                    availBal:i["availBal"].as_i64().unwrap_or(0),
                    ccy:i["ccy"].as_str().unwrap_or("").to_string(),
                    eq_usd:i["eqUsd"].as_i64().unwrap_or(0),
                    fronzen_bal:i["frozenBal"].as_i64().unwrap_or(0),
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

pub fn query_trade_account_asset<T>(api_key: T, passphrase: T, secretkey: T)
where
    T: Into<String>,
{
    println!("QueryTradeAccountAsset");
    let conf = account::apis::configuration::Configuration::new(
        "https://aws.okx.com",
        api_key,
        secretkey,
        passphrase,
    );
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async { account_api::api_v5_account_balance_get(&conf, None).await });
    // Execute account query logic here
    match res {
        Ok(response) => {
            println!("API call successful!");
            if let Some(data) = response["data"].as_array() {
                for i in data {
                    let detail = i["details"].as_array().unwrap();
                    for j in detail {
                        println!(
                            "ccy:{} eqUsd:{} fronzen:{} stgEq:{}",
                            j["ccy"], j["eqUsd"], j["frozenBal"], j["stgEq"]
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("API call failed!");
            println!("Error fetching account balance: {:?}", e);
        }
    }
}

pub fn query_finance_balance<T>(api_key: T, passphrase: T, secretkey: T)
where
    T: Into<String>,
{
    let conf = finance::apis::configuration::Configuration::new(
        "https://aws.okx.com",
        api_key,
        secretkey,
        passphrase,
    );

    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async { finance_apis::get_savings_balance(&conf, None).await });

    match res {
        Ok(response) => {
            println!("API call successful!");
            if let Some(ref etr) = response.entity {
                for v in &etr.data {
                    println!("{:?}", v)
                }
            }
        }
        Err(e) => {
            println!("API call failed!");
            println!("Error fetching account balance: {:?}", e);
        }
    }
}

pub fn query_free_asset<T>(api_key: T, passphrase: T, secretkey: T)
where
    T: Into<String>,
{
}
