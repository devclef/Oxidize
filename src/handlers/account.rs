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
