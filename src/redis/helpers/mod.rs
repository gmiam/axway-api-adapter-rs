use anyhow::Result;
use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;

pub type R2D2Con = r2d2::PooledConnection<RedisConnectionManager>;
pub const DEFAULT_TTL: usize = 15;

pub trait RedisExtensions {
    fn safe_get(&mut self, key: &str) -> Result<String>;

    fn safe_set(&mut self, key: &str, to_cache: &str, ttl: usize) -> Result<()>;
}

impl RedisExtensions for R2D2Con {
    fn safe_get(&mut self, key: &str) -> Result<String> {
        let result = self.get::<&str, String>(key)?;
        Ok(result)
    }

    fn safe_set(&mut self, key: &str, to_cache: &str, ttl: usize) -> Result<()> {
        self.set::<&str, &str, String>(key, &to_cache)?;
        self.expire::<&str, usize>("all", ttl)?;
        Ok(())
    }
}
