use actix_web::{get, web, Responder, HttpResponse};

use crate::entity::api_key::Jsonify;
use crate::cassandra::requests::api_key;
use crate::AppState;
use crate::redis::helpers::{R2D2Con, RedisExtensions};
use super::resolve_response;

#[get("/api/keys/{key_id}")]
pub async fn get_application_id_from_api_key(state: web::Data<AppState>, key_id: web::Path<String>)  -> impl Responder {
    let cassandra_pool = &state.cassandra_pool;
    let mut redis_conn: R2D2Con = state.redis_pool.get().unwrap();

    let result = redis_conn.safe_get(&key_id);
    if let Ok(response) = result {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(response)
    } else {
        let result = api_key::get_application_id_from_api_key(cassandra_pool, &key_id)
            .await
            .to_json();
        let response = resolve_response(redis_conn, &key_id, result).await;
        response
    }
}

#[get("/api/keys")]
pub async fn get_all_application_id(state: web::Data<AppState>)  -> impl Responder {
    let cassandra_pool = &state.cassandra_pool;
    let mut redis_conn: R2D2Con = state.redis_pool.get().unwrap();

    let result = redis_conn.safe_get("all");
    if let Ok(response) = result {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(response)
    } else {
        let result = api_key::get_all_application_id(cassandra_pool)
            .await
            .to_json();
        let response = resolve_response(redis_conn, "all", result).await;
        response
    }
}
