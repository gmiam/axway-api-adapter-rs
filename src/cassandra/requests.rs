use std::convert::TryFrom;

use cdrs::frame::traits::TryFromRow;

use super::exec_request;
use super::connection_pool::CassandraPool;
use super::super::entity::api_key::{ApiKeyBo, ApiKeyDto};
use actix_web::web::Data;

pub fn get_application_id_from_api_key(session: Data<CassandraPool>, key_id: String) -> Option<ApiKeyDto> {
    let cql_command = format!("SELECT * FROM bsummer.api_server_portalapikeystore where key='{}'", key_id);

    let rows = exec_request(session, cql_command)?
        .into_iter()
        .map(|it| ApiKeyBo::try_from_row(it).expect("Invalid row in result"))
        .map(|it| ApiKeyDto::from(it))
        .collect::<Vec<_>>();

    ApiKeyDto::try_from(rows).ok()
}

pub fn get_all_application_id(session: Data<CassandraPool>) -> Option<Vec<ApiKeyDto>> {
    let cql_command = format!("SELECT * FROM bsummer.api_server_portalapikeystore");

    let rows = exec_request(session, cql_command)?
        .into_iter()
        .map(|it| ApiKeyBo::try_from_row(it).expect("Invalid row in result"))
        .map(|it| ApiKeyDto::from(it))
        .collect::<Vec<ApiKeyDto>>();
    Some(rows)
}