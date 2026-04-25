use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::models::Group;
use crate::storage::Storage;

#[get("/api/groups")]
pub async fn list_groups() -> impl Responder {
    match Storage::get_all_groups() {
        Ok(groups) => HttpResponse::Ok().json(groups),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[post("/api/groups")]
pub async fn create_group(body: web::Json<Group>) -> impl Responder {
    let group = body.into_inner();

    if group.account_ids.is_empty() {
        return HttpResponse::BadRequest().body("Group must have at least one account");
    }

    match Storage::create_group(&group) {
        Ok(()) => HttpResponse::Created().json(group),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

#[put("/api/groups/{id}")]
pub async fn update_group(path: web::Path<String>, body: web::Json<Group>) -> impl Responder {
    let path_id = path.into_inner();
    let group = body.into_inner();

    if path_id != group.id {
        return HttpResponse::BadRequest().body("ID mismatch between path and body");
    }

    if group.account_ids.is_empty() {
        return HttpResponse::BadRequest().body("Group must have at least one account");
    }

    match Storage::update_group(&group) {
        Ok(()) => HttpResponse::Ok().json(group),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

#[delete("/api/groups/{id}")]
pub async fn delete_group(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    match Storage::delete_group(&id) {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}
