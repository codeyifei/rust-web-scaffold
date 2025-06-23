use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("配置文件未找到")]
    ConfigFileNotFound(#[source] ConfigError),
    #[error("无效配置")]
    InvalidConfig(#[source] ConfigError),
}
