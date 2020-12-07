extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;

use actix_web::{App, HttpServer};

use cassandra::{CassandraPool, create_cassandra_pool};

use crate::redis::connection_pool::{create_redis_pool, RedisPool};
use crate::resource::api_key::{find_all, find_by_api_key};

mod cassandra;
mod entity;
mod helpers;
mod redis;
mod service;
mod resource;

#[derive(Clone)]
pub struct AppState {
    cassandra_pool: CassandraPool,
    redis_pool: RedisPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // If we want to continue further and/or use it in production, we'll need to
    //      - Implement logging
    //      - Better Error Handling
    //      - Make (better) unit tests
    //      - benchmarks (expecially testing number of sessions and keepalive)

    let cassandra_pool = create_cassandra_pool();
    let redis_pool = create_redis_pool();

    let state = AppState {
        cassandra_pool: cassandra_pool.clone(),
        redis_pool,
    };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(find_by_api_key)
            .service(find_all)
    })
        .keep_alive(75)
        .bind("127.0.0.1:8280")?
        .run()
        .await
}
