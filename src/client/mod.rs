use crate::config::Config;
use crate::models::{AccountArray, SimpleAccount};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ACCEPT};
use log::error;

pub struct FireflyClient {
    client: reqwest::Client,
    config: Config,
}

impl FireflyClient {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Oxidize/0.1.0")
            .build()
            .unwrap();

        Self { client, config }
    }

    pub async fn get_accounts(&self, type_filter: Option<String>) -> Result<Vec<SimpleAccount>, String> {
        let mut headers = HeaderMap::new();
        if !self.config.firefly_token.is_empty() {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.config.firefly_token)).unwrap()
            );
        }
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.api+json"));

        let mut url = format!("{}/v1/accounts", self.config.firefly_url);
        if let Some(t) = type_filter {
            url = format!("{}?type={}", url, t);
        }

        let response = self.client.get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send request: {}", e);
                e.to_string()
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("API request failed with status: {}. Body: {}", status, body);
            return Err(format!("API request failed with status: {}", status));
        }

        let account_array: AccountArray = response.json()
            .await
            .map_err(|e| {
                error!("Failed to parse JSON: {}", e);
                e.to_string()
            })?;

        let simple_accounts = account_array.data.into_iter().map(|a| {
            SimpleAccount {
                name: a.attributes.name,
                balance: a.attributes.current_balance,
                currency: a.attributes.currency_symbol,
                account_type: a.attributes.account_type,
            }
        }).collect();

        Ok(simple_accounts)
    }
}
