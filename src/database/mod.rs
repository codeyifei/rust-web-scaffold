use deadpool::managed::Object;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;

#[cfg(feature = "mysql")]
pub mod mysql;

pub type Conn<C = AsyncMysqlConnection> = Object<AsyncDieselConnectionManager<C>>;
pub type Pool<C = AsyncMysqlConnection> = deadpool::managed::Pool<AsyncDieselConnectionManager<C>, Conn>;
