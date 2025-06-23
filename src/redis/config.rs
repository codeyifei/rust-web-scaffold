use crate::result::error::AppError;
use crate::types::async_trait::TryFromAsync;
use async_trait::async_trait;
use redis::aio::ConnectionManager as RedisConnectionManager;
use redis::Client;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    pub addr: String,
}

#[async_trait]
impl TryFromAsync<Config> for RedisConnectionManager {
    type Error = AppError;

    async fn try_from_async(config: Config) -> Result<Self, Self::Error> {
        let redis_client =
            Client::open(config.addr.clone()).expect("创建redis客户端失败");
        let redis_manager = RedisConnectionManager::new(redis_client).await?;

        Ok(redis_manager)
    }
}