use std::env;
use std::sync::Arc;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::QueryExecutor;
use cdrs::types::prelude::Row;
use dotenv::dotenv;

pub mod api_key;

pub type CassandraPool = Arc<Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>>;

fn exec_request(session: &CassandraPool, cql_command: &str) -> anyhow::Result<Vec<Row>> {
    session.query(cql_command)
        ?.get_body()
        ?.into_rows()
        .ok_or_else(|| anyhow::Error::msg(""))
}

pub fn create_cassandra_pool() -> CassandraPool {
    dotenv().ok();
    let cassandra_host = env::var("CASSANDRA_HOST").unwrap();
    let cassandra_port = env::var("CASSANDRA_PORT").unwrap();

    let auth = NoneAuthenticator;
    let connection_string = format!("{}:{}", cassandra_host, cassandra_port);
    println!("here is the connection string {}", connection_string);
    let node = NodeTcpConfigBuilder::new(&connection_string, auth).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    Arc::new(new_session(&cluster_config, RoundRobin::new()).expect("session should be created"))
}