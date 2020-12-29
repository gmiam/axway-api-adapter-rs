extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;


use actix_web::{App, HttpServer, middleware::Logger};
use crate::cassandra::{CassandraPool, create_cassandra_pool};
use env_logger::Env;
use crate::redis::connection_pool::{create_redis_pool, RedisPool};
use crate::resource::api_key;

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
    //      - benchmarks (especially testing number of sessions and keepalive)

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let cassandra_pool = create_cassandra_pool();
    let redis_pool = create_redis_pool();

    let state = AppState {
        cassandra_pool: cassandra_pool.clone(),
        redis_pool,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(state.clone())
            .service(api_key::find_by_api_key)
            .service(api_key::find_all)
    })
        .keep_alive(75)
        .bind("0.0.0.0:8280")?
        .run()
        .await
}
