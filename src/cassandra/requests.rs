use std::convert::TryFrom;

use cdrs::frame::traits::TryFromRow;

use super::exec_request;
use super::connection_pool::CassandraPool;
use super::super::entity::api_key::{ApiKeyRawStruct, ApiKeyStruct};
use actix_web::web::Data;

pub fn get_application_id_from_api_key(session: Data<CassandraPool>, key_id: String) -> Option<ApiKeyStruct> {
    let cql_command = format!("SELECT * FROM bsummer.api_server_portalapikeystore where key='{}'", key_id);

    let rows = exec_request(session, cql_command)?
        .into_iter()
        .map(|it| ApiKeyRawStruct::try_from_row(it).expect("Invalid row in result"))
        .map(|it| ApiKeyStruct::from(it))
        .collect::<Vec<_>>();

    ApiKeyStruct::try_from(rows).ok()
}