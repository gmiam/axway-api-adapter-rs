use actix_web::{get, Responder, web};

use super::super::{AppState, service::api_key::AppIdService};

#[get("/api/keys/{key_id}")]
pub async fn find_by_api_key(state: web::Data<AppState>, key_id: web::Path<String>) -> impl Responder {
    AppIdService::find_by_api_key(state, key_id.to_string()).await
}

#[get("/api/keys")]
pub async fn find_all(state: web::Data<AppState>) -> impl Responder {
    AppIdService::find_all(state).await
}