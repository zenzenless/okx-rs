

use reqwest::header;

use crate::model;

pub fn get_coins_map() -> Result<Vec<model::CoinsInfo>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("Referer", "https://petstore.swagger.io/".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Not A(Brand\";v=\"8\", \"Chromium\";v=\"132\", \"Brave\";v=\"132\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    let res= reqwest::blocking::Client::new().get("https://api.coingecko.com/api/v3/coins/list").headers(headers).send().unwrap();
    let info=res.json::<Vec<model::CoinsInfo>>()?;
    Ok(info)
}


pub fn get_coins_info(coin_id:String) -> Result<serde_json::Value, Box<dyn std::error::Error>>{
        let mut headers = header::HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());

    let res = reqwest::blocking::Client::new()
        .get("https://api.coingecko.com/api/v3/coins/bitcoin?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false")
        .headers(headers)
        .send()?
        .text()?;
    let data = serde_json::from_str(&res)?;
    Ok(data)
}

pub fn get_coins_prices(coins_id:&str,currencies:&str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());
    let url=format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",coins_id,currencies);
    let res = reqwest::blocking::Client::new()
        .get(url)
        .headers(headers)
        .send()?
        .text()?;
    serde_json::from_str(&res).map_err(|e| e.into())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coins_map() {
        let result = get_coins_map();
        let list=result.unwrap();
        //check list have repeat symbol
        let mut symbols=Vec::new();
        for item in list {
            if symbols.contains(&item.symbol) {
                panic!("repeat symbol: {}", item.id);
            }
            symbols.push(item.symbol);
        }
    }

    #[test]
    fn test_get_coins_info() {
        let result = get_coins_info("bitcoin".to_string());
        assert!(result.is_ok());
    }
}

