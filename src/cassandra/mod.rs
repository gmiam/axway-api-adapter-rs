use std::sync::Arc;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::QueryExecutor;
use cdrs::types::prelude::Row;

pub mod api_key;

pub type CassandraPool = Arc<Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>>;

fn exec_request(session: &CassandraPool, cql_command: &str) -> anyhow::Result<Vec<Row>> {
    session.query(cql_command)
        ?.get_body()
        ?.into_rows()
        .ok_or_else(|| anyhow::Error::msg(""))
}

pub fn create_cassandra_pool() -> CassandraPool {
    let auth = NoneAuthenticator;
    let node = NodeTcpConfigBuilder::new("localhost:9042", auth).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    Arc::new(new_session(&cluster_config, RoundRobin::new()).expect("session should be created"))
}