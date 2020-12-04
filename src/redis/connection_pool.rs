use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn create_redis_pool() -> Pool<RedisConnectionManager> {
    let manager = r2d2_redis::RedisConnectionManager::new("redis://127.0.0.1/").unwrap();
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool for redis.")
}
