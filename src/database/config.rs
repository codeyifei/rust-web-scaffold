use crate::types::async_trait::TryFromAsync;
#[cfg(feature = "sea_orm_mysql")]
use async_trait::async_trait;
#[cfg(feature = "diesel_mysql")]
use diesel::r2d2::{ConnectionManager, Pool};
#[cfg(feature = "diesel_mysql")]
use diesel::MysqlConnection;
use log::LevelFilter;
#[cfg(feature = "sea_orm_mysql")]
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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

#[cfg(feature = "sea_orm_mysql")]
#[async_trait]
impl TryFromAsync<MysqlConfig> for DatabaseConnection {
    type Error = DbErr;

    async fn try_from_async(config: MysqlConfig) -> Result<Self, Self::Error> {
        let mut opt = ConnectOptions::new(config.addr);
        opt.min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .sqlx_logging(config.logging)
            .sqlx_logging_level(
                log::LevelFilter::from_str(config.logging_filter.as_str())
                    .unwrap_or(LevelFilter::Debug),
            );

        let db = Database::connect(opt).await?;
        // if config.auto_migration {
        //     Migrator::up(&db, None).await?;
        // }

        Ok(db)
    }
}

#[cfg(feature = "diesel_mysql")]
impl TryFrom<MysqlConfig> for Pool<ConnectionManager<MysqlConnection>> {
    type Error = AppError;

    fn try_from(config: MysqlConfig) -> Result<Self, Self::Error> {
        let manager = ConnectionManager::<MysqlConnection>::new(config.addr);
        let conn_pool = Pool::builder().build(manager)?;
        
        Ok(conn_pool)
    }
}

// #[async_trait]
// impl TryFromAsync<MysqlConfig> for Pool<ConnectionManager<MysqlConnection>> {
//     type Error = AppError;
// 
//     async fn try_from_async(config: MysqlConfig) -> Result<Self, Self::Error> {
//         let pool = MySqlPoolOptions::new()
//             .max_connections(5)
//             .connect(config.addr.as_str()).await?;
//     }
// }
