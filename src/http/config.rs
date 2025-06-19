use crate::types::async_trait::TryFromAsync;
use async_trait::async_trait;
use serde::Deserialize;
use std::io;
use tokio::net::TcpListener;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_server_addr")]
    pub addr: String,
    #[serde(default = "default_timeout_seconds")]
    pub timeout: u64,
}

fn default_server_addr() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_timeout_seconds() -> u64 { 30 }

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: default_server_addr(),
            timeout: default_timeout_seconds(),
        }
    }
}

#[async_trait]
impl TryFromAsync<Config> for TcpListener {
    type Error = io::Error;

    async fn try_from_async(config: Config) -> Result<Self, Self::Error> {
        Ok(TcpListener::bind(config.addr).await?)
    }
}