use async_trait::async_trait;
use deadpool::managed::Object;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;
use crate::result::error::AppError;

#[cfg(feature = "mysql")]
pub mod mysql;

pub type Conn<C = AsyncMysqlConnection> = Object<AsyncDieselConnectionManager<C>>;
pub type Pool<C = AsyncMysqlConnection> = deadpool::managed::Pool<AsyncDieselConnectionManager<C>, Conn>;

#[async_trait]
pub trait DbGetter {
    async fn db(&self) -> Result<Conn, AppError>;
}
