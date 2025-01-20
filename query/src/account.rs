use account::apis::account_api;
use finance::apis::finance_apis;
use fundings::apis::fundings_api;

pub fn query_account_asset<T>(api_key:T,passphrase:T,secretkey:T)
where T: Into<String>
 {
    println!("QueryAccountAsset");
let conf = fundings::apis::configuration::Configuration::new(
                "https://aws.okx.com",
                api_key,
                secretkey,
                passphrase,
            );
            let rt = tokio::runtime::Runtime::new().unwrap();
            let res =
                rt.block_on(async { fundings_api::api_v5_asset_balances_get(&conf, None).await });
            // Execute account query logic here
            match res {
                Ok(response) => {
                    println!("API call successful!");
                    let my_asset =response["data"].as_array().unwrap();
                    for i in my_asset{

                        println!("{}:{}:fronzen:{}", i["ccy"],i["availBal"],i["frozenBal"]);
                    }
                }
                Err(e) => {
                    println!("API call failed!");
                    println!("Error fetching account balance: {:?}", e);
                }
            }
}

pub fn query_trade_account_asset<T>(api_key:T, passphrase:T, secretkey:T)
where T: Into<String>
 {
    println!("QueryTradeAccountAsset");
let conf = account::apis::configuration::Configuration::new(
            "https://aws.okx.com",
                api_key,
                secretkey,
                passphrase,
            );
            let rt = tokio::runtime::Runtime::new().unwrap();
            let res =
                rt.block_on(async { account_api::api_v5_account_balance_get(&conf, None).await });
            // Execute account query logic here
            match res {
                Ok(response) => {
                    println!("API call successful!");
                    if let Some(data)=response["data"].as_array(){
                        for i in data{
                            let detail= i["details"].as_array().unwrap();
                            for j in detail{
                                println!("ccy:{} eqUsd:{} fronzen:{} stgEq:{}", j["ccy"], j["eqUsd"], j["frozenBal"],j["stgEq"]);
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


pub fn query_finance_balance<T>(api_key:T, passphrase:T, secretkey:T)
where T: Into<String>
{
   let conf = finance::apis::configuration::Configuration::new(
                "https://aws.okx.com",
                api_key,
                secretkey,
                passphrase,
            );

    let rt = tokio::runtime::Runtime::new().unwrap();
    let res =
        rt.block_on(async { finance_apis::get_savings_balance(&conf, None).await });
    
    match res {
        Ok(response) => {
            println!("API call successful!");
            if let Some(ref etr) = response.entity{
                for v in &etr.data{
                    println!("{:?}",v)
                }
            } 
        }
        Err(e) => {
            println!("API call failed!");
            println!("Error fetching account balance: {:?}", e);
        }
    }
            
}