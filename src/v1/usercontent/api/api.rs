use worker::console_log;
use crate::v1::usercontent::api::types::{FeedOptions, FeedResult};

const BASE_URL: &str = "https://live.warthunder.com";

pub async fn get_feed(options: &FeedOptions) -> Result<FeedResult, String> {
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/api/feed/get_unlogged/", BASE_URL))
        .form(&options)
        .send()
        .await;

    match res {
        Ok(res) => {
            let status = res.status();
            if status.is_success() {
                let body = res.text().await.unwrap();
                let parsed_response: FeedResult = match serde_json::from_str(&body) {
                    Ok(data) => data,
                    Err(e) => return Err(e.to_string())
                };

                match parsed_response.status == "OK" {
                    true => Ok(parsed_response),
                    false => {
                        console_log!("Failed to get feed: {:?}", parsed_response);
                        Err(body)
                    }
                }
            } else {
                Err(format!("Request failed with status code: {}", status))
            }
        },
        Err(e) => Err(e.to_string())
    }
}