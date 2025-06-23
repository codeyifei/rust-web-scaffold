use crate::database::Pool;
use crate::result::error::AppError;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MysqlConfig {
    pub addr: String,
    #[serde(default = "default_mysql_max_connections")]
    pub max_connections: u32,
}

fn default_mysql_max_connections() -> u32 {
    100
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
