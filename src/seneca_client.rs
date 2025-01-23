use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub struct SenecaClient {
    client: Client,
    access_key: String,
}

impl SenecaClient {
    pub fn new(access_key: String) -> Self {
        let client = Client::new();
        println!("SenecaClient created");
        SenecaClient { client, access_key }
    }

    pub async fn get_user_id(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://user-info.app.senecalearning.com/api/user-info/me".to_string();

        let headers: HeaderMap = [
            ("Host", "user-info.app.senecalearning.com"),
            (
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0",
            ),
            ("Accept", "*/*"),
            ("Accept-Language", "en-GB,en;q=0.5"),
            ("Accept-Encoding", "gzip, deflate, br, zstd"),
            ("Referer", "https://app.senecalearning.com/"),
            ("access-key", &self.access_key),
            ("Content-Type", "application/json"),
            (
                "correlationId",
                "1737330516472::76115c42-02c9-4d56-0000-000000000000",
            ),
            ("user-region", "GB"),
            ("Origin", "https://app.senecalearning.com"),
            ("DNT", "1"),
            ("Sec-GPC", "1"),
            ("Sec-Fetch-Dest", "empty"),
            ("Sec-Fetch-Mode", "cors"),
            ("Sec-Fetch-Site", "same-site"),
            ("Connection", "keep-alive"),
            ("host", "user-info.app.senecalearning.com"),
        ]
        .iter()
        .map(|(key, value)| (key.parse().unwrap(), HeaderValue::from_str(value).unwrap()))
        .collect();

        let response = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.json::<Value>().await?;
            Ok(body["userId"].as_str().unwrap().to_string())
        } else {
            Err(Box::new(response.error_for_status().unwrap_err()))
        }
    }
}
