use anyhow::{Error, Result};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub error: Option<Vec<String>>,
    pub result: Option<T>,
}

impl<T: DeserializeOwned> ApiResponse<T> {
    pub async fn get(url: &str) -> Result<T> {
        let url = String::from(url);
        let url = Url::parse(url.as_str())?;

        let response = reqwest::get(url).await?.json::<ApiResponse<T>>().await?;

        match response.result {
            Some(r) => Ok(r),
            None => Err(Error::msg(serde_json::to_string(&response.error.unwrap())?)),
        }
    }
}
