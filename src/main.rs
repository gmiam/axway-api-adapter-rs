extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;

mod cassandra;
mod helpers;
mod entity;
mod service;

use crate::cassandra::connection_pool::create_cassandra_pool;
use crate::service::api_key::get_application_id_from_api_key;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create project structure
    // Try Redis
    // Add other requests
    // If we want to continue further and/or use it in production, we'll need to
    //      - Implement logging
    //      - Make better unit tests
    //      - benchmarks (expecially testing number of sessions and keepalive)
  let session = create_cassandra_pool();

    HttpServer::new(move || {
        App::new()
            .data(session.clone())
            .service(get_application_id_from_api_key)
    })
        .keep_alive(75)
        .bind("127.0.0.1:8280")?
        .run()
        .await
}
