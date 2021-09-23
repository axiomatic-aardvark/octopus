use anyhow::{bail, Error, Result};
use base64::{decode, encode};
use dotenv::dotenv;
use hmac::{Hmac, Mac, NewMac};
use http::{HeaderMap, HeaderValue};
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::api_response::ApiResponse;

type HmacSha512 = Hmac<Sha512>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Online,
    Maintenance,
    #[serde(rename = "cancel_only")]
    CancelOnly,
    #[serde(rename = "post_only")]
    PostOnly,
}

#[derive(Serialize, Deserialize)]
struct Order {
    #[serde(rename = "refid")]
    ref_id: Option<String>,
    #[serde(rename = "userref")]
    user_ref: i32,
    status: Status,
    #[serde(rename = "opentm")]
    open_tm: f32,
    #[serde(rename = "starttm")]
    start_tm: f32,
    #[serde(rename = "expiretm")]
    expire_tm: f32,
    descr: OrderDescription,
    vol: f32,
    vol_exec: f32,
    cost: f32,
    fee: f32,
    price: f32,
    #[serde(rename = "stopprice")]
    stop_price: f32,
    #[serde(rename = "limitprice")]
    limit_price: f32,
    misc: String,
    oflags: String,
    trades: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
    SettlePosition,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Type {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize)]
struct OrderDescription {
    pair: String,
    #[serde(rename = "type")]
    kind: Type,
    #[serde(rename = "ordertype")]
    order_type: OrderType,
    price: f32,
    price2: f32,
    leverage: String,
    order: String,
    close: String,
}

#[derive(Serialize, Deserialize)]
pub struct Orders {
    open: HashMap<String, Order>,
}

#[derive(Serialize, Deserialize, Clone)]
struct RequestData {
    nonce: i64,
    trades: bool,
}

fn get_signature(
    api_path: String,
    nonce: String,
    url_encoded_body: String,
    api_secret: String,
) -> Result<String> {
    let hash_digest = Sha256::digest(format!("{}{}", nonce, url_encoded_body).as_bytes());
    let private_key = decode(&api_secret)?;
    let mut mac = match HmacSha512::new_varkey(&private_key) {
        Ok(m) => m,
        Err(e) => bail!(e),
    };
    let mut hmac_data = api_path.into_bytes();

    hmac_data.append(&mut hash_digest.to_vec());
    mac.update(&hmac_data);
    Ok(encode(mac.finalize().into_bytes()))
}

impl Orders {
    pub async fn get() -> Result<Self> {
        dotenv().ok();
        let api_key = dotenv!("API_KEY");
        let api_secret = dotenv!("API_SECRET");
        let otp = dotenv!("OTP");

        let api_url = "https://api.kraken.com";
        let uri_path = "/0/private/OpenOrders";

        let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        let payload_body = format!("nonce={}&otp={}", &nonce.to_string(), otp.to_string());

        let sign = get_signature(
            uri_path.to_string(),
            nonce.to_string(),
            payload_body.to_owned(),
            String::from(api_secret),
        )?;

        let mut headers = HeaderMap::new();
        headers.insert("API-Key", HeaderValue::from_str(&api_key.to_string())?);
        headers.insert("API-Sign", HeaderValue::from_str(&sign.to_string())?);

        let url = Url::parse((api_url.to_owned() + uri_path).as_str())?;

        let client = reqwest::Client::builder().build()?;
        let res = client
            .post(url)
            .body(payload_body)
            .headers(headers)
            .send()
            .await?;

        let response: ApiResponse<Orders> = serde_json::from_str(&res.text().await?)?;
        match response.result {
            Some(r) => Ok(r),
            None => Err(Error::msg(serde_json::to_string(&response.error.unwrap())?)),
        }
    }
}
