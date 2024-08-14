use iconf::configs;
use num_bigfloat::BigFloat;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    status: Status,
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    timestamp: String,
    error_code: i32,
    error_message: Option<String>,
    elapsed: i32,
    credit_count: i32,
    notice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    MATIC: Vec<CoinData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CoinData {
    id: i64,
    name: String,
    symbol: String,
    slug: String,
    num_market_pairs: i32,
    date_added: String,
    tags: Vec<Tag>,
    max_supply: Option<f64>,
    circulating_supply: f64,
    total_supply: f64,
    is_active: i32,
    infinite_supply: bool,
    platform: Option<Platform>,
    cmc_rank: i32,
    is_fiat: i32,
    self_reported_circulating_supply: Option<f64>,
    self_reported_market_cap: Option<f64>,
    tvl_ratio: Option<f64>,
    last_updated: String,
    quote: Quote,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    slug: String,
    name: String,
    category: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Platform {
    // 根据你的需求添加字段
}

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    USD: QuoteData,
}

#[derive(Debug, Serialize, Deserialize)]
struct QuoteData {
    price: f64,
    volume_24h: f64,
    volume_change_24h: f64,
    percent_change_1h: f64,
    percent_change_24h: f64,
    percent_change_7d: f64,
    percent_change_30d: f64,
    percent_change_60d: f64,
    percent_change_90d: f64,
    market_cap: f64,
    market_cap_dominance: f64,
    fully_diluted_market_cap: f64,
    tvl: Option<f64>,
    last_updated: String,
}

const SYMBOL_MATIC:&str = "MATIC";

pub async fn sync_coin_price() -> Result<(), String>{
    unsafe {
        let url = configs::get_str("coinmarketcap", "url");
        let key = configs::get_str("coinmarketcap", "key");

        let client = Client::new();

        let mut headers = HeaderMap::new();
        headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(&key).unwrap());

        let response_ = client
            .get(format!("{}?symbol=MATIC&convert=USD",url.as_str()))
            .headers(headers)
            .send().await;

        let response = response_.unwrap();

        if response.status().is_success() {
            let body = response.text().await.unwrap();
            debug!("Response body: {}", body.clone());

            let parsed: ApiResponse = serde_json::from_str(body.as_str()).unwrap();
            debug!("{:?}", parsed);

            let price = parsed.data.MATIC[0].quote.USD.price;
            debug!("matic={}", price);

            let cac = common::cache::common_rds::set_string(SYMBOL_MATIC.to_string(), format!("{}", price).to_string()).await;
            if cac.is_err() {
                tracing::warn!("{:?}", cac);
            }

        } else {
            println!("Request failed with status: {}", response.status());
        }

        Ok(())
    }
}

pub async fn get_price_by_matic(amount: String) -> Result<String, String> {
    let unit = common::cache::common_rds::get_string(SYMBOL_MATIC.to_string()).await;
    if unit.is_err() {
        return Err(unit.err().unwrap().to_string());
    }

    let u = unit.unwrap();
    if "" == u {
        return Err("matic price not found by cache".to_string());
    }

    let a = BigFloat::parse(amount.as_str()).unwrap();
    let b = BigFloat::parse(u.as_str()).unwrap();

    let p = a.mul(&b);
    Ok(format!("{:.10}", p.to_f64()))
}