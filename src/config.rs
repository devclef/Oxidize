use std::env;
use dotenv::dotenv;

#[derive(Clone, Debug)]
pub struct Config {
    pub firefly_url: String,
    pub firefly_token: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let firefly_url = env::var("FIREFLY_III_URL")
            .unwrap_or_else(|_| "https://demo.firefly-iii.org/api".to_string());
        let firefly_token = env::var("FIREFLY_III_ACCESS_TOKEN")
            .unwrap_or_else(|_| "".to_string());
        let host = env::var("HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);

        Self {
            firefly_url,
            firefly_token,
            host,
            port,
        }
    }
}
