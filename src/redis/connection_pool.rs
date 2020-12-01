use r2d2_redis::RedisConnectionManager;
use r2d2::Pool;

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn create_redis_pool() -> Pool<RedisConnectionManager> {
    let manager = r2d2_redis::RedisConnectionManager::new("redis://127.0.0.1/").unwrap();
    let redis_pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool for redis.");
    redis_pool
}