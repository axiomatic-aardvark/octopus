use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServerTime {
    #[serde(rename = "unixtime")]
    pub unix_time: i32,
    rfc1123: String,
}
