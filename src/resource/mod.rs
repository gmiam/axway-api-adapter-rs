use actix_web::HttpResponse;
use actix_web::web::Data;
use anyhow::Result;
use async_trait::async_trait;

use crate::AppState;
use crate::redis::helpers::{DEFAULT_TTL, RedisExtensions};
use crate::resource::error::{AdapterHttpResponses, error_500};

pub mod api_key;
pub mod error;

#[async_trait]
pub trait MaybeCache {
    async fn then_cache(self, state: Data<AppState>, key: &str) -> HttpResponse;
}

#[async_trait]
impl MaybeCache for Result<String> {
    async fn then_cache(self, state: Data<AppState>, key: &str) -> HttpResponse {
        if let Ok(response) = self {
            state.redis_pool.get()
                .map_err(anyhow::Error::msg)
                .and_then(|mut conn| conn.safe_set(key, &response, DEFAULT_TTL))
                .map(|_| response.ok_200())
                .unwrap_or_else(|_| error_500())
        } else { error_500() }
    }
}