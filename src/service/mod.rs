use crate::redis::helpers::{R2D2Con, RedisExtensions, DEFAULT_TTL};
use anyhow::Result;
use actix_web::HttpResponse;

pub mod api_key;

pub async fn resolve_response(mut redis_conn: R2D2Con, key: &str, result: Result<String>) -> HttpResponse {
    if let Ok(response) = result {
        redis_conn.safe_set(key, &response, DEFAULT_TTL).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(response)
    } else {
        HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body("Internal Server Error")
    }
}