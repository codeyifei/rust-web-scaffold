use crate::result::error::AppError;
use config::{Config, File};
use dotenv::dotenv;
use serde::de::DeserializeOwned;
use std::env;

pub mod error;

pub fn init<T>() -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    dotenv().ok();

    let config_file = env::var("CONFIG_FILE").unwrap_or("config.toml".to_string());
    let local_config_file = env::var("LOCAL_CONFIG_FILE").unwrap_or("config.local.toml".to_string());
    let app_config = Config::builder()
        .add_source(File::with_name(config_file.as_str()).required(true))
        .add_source(File::with_name(local_config_file.as_str()).required(false))
        .build()?
        .try_deserialize()?;

    Ok(app_config)
}
