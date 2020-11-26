use std::sync::Arc;

use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::authenticators::NoneAuthenticator;
use cdrs::load_balancing::RoundRobin;

pub type CassandraPool = Arc<Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>>;

pub fn create_cassandra_pool() -> CassandraPool {
    let auth = NoneAuthenticator;
    let node = NodeTcpConfigBuilder::new("localhost:9042", auth).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    Arc::new(new_session(&cluster_config, RoundRobin::new()).expect("session should be created"))
}
