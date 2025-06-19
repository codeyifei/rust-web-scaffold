use redis::aio::ConnectionManager as RedisConnectionManager;

pub mod config;
pub mod types;

pub trait RdsGetter {
    fn rds(&self) -> RedisConnectionManager;
}
