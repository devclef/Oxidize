use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ACCEPT};
use std::env;
use dotenv::dotenv;
use log::{info, error};

#[derive(Serialize, Deserialize, Debug)]
struct AccountArray {
    data: Vec<AccountRead>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountRead {
    id: String,
    attributes: AccountAttributes,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountAttributes {
    name: String,
    current_balance: String,
    currency_symbol: String,
}

#[derive(Serialize)]
struct SimpleAccount {
    name: String,
    balance: String,
    currency: String,
}

#[get("/api/accounts")]
async fn get_accounts() -> impl Responder {
    let api_url = env::var("FIREFLY_III_URL").unwrap_or_else(|_| "https://demo.firefly-iii.org/api".to_string());
    let api_token = env::var("FIREFLY_III_ACCESS_TOKEN").unwrap_or_else(|_| "".to_string());

    let client = reqwest::Client::builder()
        .user_agent("Oxidize/0.1.0")
        .build()
        .unwrap();

    let mut headers = HeaderMap::new();
    if !api_token.is_empty() {
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_token)).unwrap());
    }
    headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.api+json"));

    match client.get(format!("{}/v1/accounts", api_url))
        .headers(headers)
        .send()
        .await {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<AccountArray>().await {
                        Ok(accounts) => {
                            let simple_accounts: Vec<SimpleAccount> = accounts.data.into_iter().map(|a| {
                                SimpleAccount {
                                    name: a.attributes.name,
                                    balance: a.attributes.current_balance,
                                    currency: a.attributes.currency_symbol,
                                }
                            }).collect();
                            HttpResponse::Ok().json(simple_accounts)
                        }
                        Err(e) => {
                            error!("Failed to parse JSON: {}", e);
                            HttpResponse::InternalServerError().body(format!("Failed to parse JSON: {}", e))
                        }
                    }
                } else {
                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    error!("API request failed with status: {}. Body: {}", status, body);
                    HttpResponse::InternalServerError().body(format!("API request failed with status: {}", status))
                }
            }
            Err(e) => {
                error!("Failed to send request: {}", e);
                HttpResponse::InternalServerError().body(format!("Failed to send request: {}", e))
            }
        }
}

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port_str.parse::<u16>().unwrap_or(8080);

    info!("Starting server at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(get_accounts)
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind((host, port))?
    .run()
    .await
}
