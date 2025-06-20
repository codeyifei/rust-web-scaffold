use crate::database::Pool;
use crate::result::error::AppError;
use diesel_async::AsyncMysqlConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use log::{LevelFilter, debug, info};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MysqlConfig {
    pub addr: String,
    #[serde(default = "default_mysql_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_mysql_max_connections")]
    pub max_connections: u32,
    #[serde(
        default = "default_mysql_connect_timeout",
        deserialize_with = "from_seconds"
    )]
    pub connect_timeout: Duration,
    #[serde(default)]
    pub auto_migration: bool,
    #[serde(default)]
    pub logging: bool,
    #[serde(default = "default_mysql_log_level")]
    pub logging_filter: String,
}

fn default_mysql_log_level() -> String {
    LevelFilter::Info.to_string()
}

fn default_mysql_min_connections() -> u32 {
    5
}

fn default_mysql_max_connections() -> u32 {
    100
}

fn default_mysql_connect_timeout() -> Duration {
    Duration::from_secs(30)
}

// 自定义反序列化函数（秒）
fn from_seconds<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let secs: u64 = Deserialize::deserialize(deserializer)?;
    Ok(Duration::from_secs(secs))
}

impl TryFrom<MysqlConfig> for Pool {
    type Error = AppError;

    fn try_from(cfg: MysqlConfig) -> Result<Self, Self::Error> {
        let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(&cfg.addr);
        debug!("正在建立数据库链接...");
        let pool = deadpool::managed::Pool::builder(config)
            .max_size(cfg.max_connections.try_into()?)
            .build()?;
        info!("数据库链接建立成功");

        Ok(pool)
    }
}
