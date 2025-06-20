#[cfg(feature = "rds")]
use redis::aio::ConnectionManager as RedisConnectionManager;

pub mod config;
pub mod types;

#[cfg(feature = "rds")]
pub trait RdsGetter {
    fn rds(&self) -> RedisConnectionManager;
}
