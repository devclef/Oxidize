use actix_web::{get, web, HttpResponse, Responder, HttpRequest};
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
    req: HttpRequest,
) -> impl Responder {
    let query_string = req.query_string();
    let params: Vec<(String, String)> = serde_urlencoded::from_str(query_string).unwrap_or_default();

    let account_ids: Vec<String> = params.into_iter()
        .filter(|(k, _)| k == "accounts[]")
        .map(|(_, v)| v)
        .collect();

    let ids = if account_ids.is_empty() {
        None
    } else {
        Some(account_ids)
    };

    match client.get_balance_history(ids).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
