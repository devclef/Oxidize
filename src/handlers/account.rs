use actix_web::{get, web, HttpResponse, Responder};
use crate::client::FireflyClient;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountQuery {
    #[serde(rename = "type")]
    pub account_type: Option<String>,
}

#[get("/api/accounts")]
pub async fn get_accounts(
    client: web::Data<FireflyClient>,
    query: web::Query<AccountQuery>,
) -> impl Responder {
    match client.get_accounts(query.account_type.clone()).await {
        Ok(accounts) => HttpResponse::Ok().json(accounts),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[get("/api/accounts/balance-history")]
pub async fn get_balance_history(
    client: web::Data<FireflyClient>,
) -> impl Responder {
    match client.get_asset_balance_history().await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
