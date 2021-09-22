use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PairInfo {
    #[serde(rename = "altname")]
    alt_name: String,
    #[serde(rename = "wsname")]
    ws_name: String,
    aclass_base: String,
    base: String,
    aclass_quote: String,
    quote: String,
    lot: String,
    pair_decimals: i32,
    lot_decimals: i32,
    lot_multiplier: i32,
    leverage_buy: Vec<i32>,
    leverage_sell: Vec<i32>,
    fees: Vec<(i32, f32)>,
    fees_maker: Vec<(i32, f32)>,
    fee_volume_currency: String,
    margin_call: i32,
    margin_stop: i32,
    #[serde(rename = "ordermin")]
    order_min: String,
}

#[derive(Serialize, Deserialize)]
pub struct XbtUsd {
    #[serde(rename = "XXBTZUSD")]
    pair_info: PairInfo,
}
