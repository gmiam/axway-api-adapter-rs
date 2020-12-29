use std::env;
use dotenv::dotenv;

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn create_redis_pool() -> Pool<RedisConnectionManager> {
    let redis_host = env::var("REDIS_HOST").unwrap();
    let redis_port = env::var("REDIS_PORT").unwrap();

    let manager = r2d2_redis::RedisConnectionManager::new(format!("redis://{}/", redis_host)).unwrap();
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool for redis.")
}
