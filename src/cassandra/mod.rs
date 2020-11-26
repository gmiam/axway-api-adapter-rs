use cdrs::types::prelude::Row;
use crate::cassandra::connection_pool::CassandraPool;
use cdrs::query::QueryExecutor;
use actix_web::web::Data;

pub mod connection_pool;
pub mod requests;

pub fn exec_request(session: Data<CassandraPool>, cql_command: String) -> Option<Vec<Row>> {
    session.query(cql_command).ok()?
        .get_body().ok()?
        .into_rows()
}