use log4rs::config::runtime;
use crate::log::config::Config;
use crate::result::error::AppError;

pub mod config;

pub fn init(config: Config) -> Result<(), AppError> {
    let log_config: runtime::Config = config.try_into()?;
    log4rs::init_config(log_config)?;
    
    Ok(())
}
