use actix_web::{get, web, Responder, HttpResponse};

use crate::cassandra::connection_pool::CassandraPool;
use crate::cassandra::requests;

#[get("/api/keys/{key_id}")]
pub async fn get_application_id_from_api_key(session: web::Data<CassandraPool>, key_id: web::Path<String>)  -> impl Responder {
    let now = std::time::Instant::now();

    let response = requests::get_application_id_from_api_key(session, key_id.into_inner()).unwrap();
    println!("{}", now.elapsed().as_millis());

    HttpResponse::Ok().json(response)

}