use std::convert::TryInto;

use cdrs::frame::traits::TryFromRow;
use itertools::Itertools;

use crate::cassandra::{CassandraPool, exec_request};
use crate::entity::api_key::{ApiKeyBo, ApiKeyDto};

pub struct AppIdRepository;

impl AppIdRepository {
    const FIND_ALL_CMD: &'static str = "SELECT * FROM bsummer.api_server_portalapikeystore";

    pub async fn find_by_api_key(pool: &CassandraPool, key_id: &str) -> anyhow::Result<ApiKeyDto> {
        exec_request(pool, &Self::find_by_key_cmd(key_id))?.into_iter()
            .map(|it| ApiKeyBo::try_from_row(it).expect("Invalid row in result"))
            .map(ApiKeyDto::from)
            .collect_vec()
            .try_into()
    }

    pub async fn find_all(pool: &CassandraPool) -> anyhow::Result<Vec<ApiKeyDto>> {
        Ok(exec_request(pool, Self::FIND_ALL_CMD)?.into_iter()
            .map(|it| ApiKeyBo::try_from_row(it).expect("Invalid row in result"))
            .map(ApiKeyDto::from)
            .collect_vec())
    }

    fn find_by_key_cmd(key_id: &str) -> String {
        format!("SELECT * FROM bsummer.api_server_portalapikeystore where key='{}'", key_id)
    }
}