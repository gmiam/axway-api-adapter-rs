use actix_web::{Responder, web::Data};

use crate::{
    AppState,
    cassandra::api_key::AppIdRepository,
    entity::api_key::Jsonify,
    redis::helpers::RedisExtensions,
    resource::{error::AdapterHttpResponses, MaybeCache},
};

pub struct AppIdService;

impl AppIdService {
    const FIND_ALL_KEY: &'static str = "all";

    pub async fn find_by_api_key(state: Data<AppState>, key_id: String) -> impl Responder {
        if let Ok(response) = Self::maybe_from_cache(&state, &key_id) {
            response.ok_200()
        } else {
            AppIdRepository::find_by_api_key(&state.cassandra_pool, &key_id).await
                .to_json()
                .then_cache(state, &key_id).await
        }
    }

    pub async fn find_all(state: Data<AppState>) -> impl Responder {
        if let Ok(response) = Self::maybe_from_cache(&state, Self::FIND_ALL_KEY) {
            response.ok_200()
        } else {
            AppIdRepository::find_all(&state.cassandra_pool).await
                .to_json()
                .then_cache(state, Self::FIND_ALL_KEY).await
        }
    }

    fn maybe_from_cache(state: &Data<AppState>, key: &str) -> anyhow::Result<String> {
        state.redis_pool.get()
            .map_err(anyhow::Error::msg)
            .and_then(|mut conn| conn.safe_get(key))
    }
}