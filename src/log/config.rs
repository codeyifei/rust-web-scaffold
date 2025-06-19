use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use crate::result::error::AppError;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default)]
    pub enable_file: bool,
    #[serde(default = "default_log_file_path")]
    pub file_path: String,
}

fn default_log_level() -> String {
    LevelFilter::Info.to_string()
}

fn default_log_file_path() -> String {
    "log".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            enable_file: false,
            file_path: default_log_file_path(),
        }
    }
}

impl TryFrom<Config> for log4rs::config::runtime::Config {
    type Error = AppError;

    fn try_from(cfg: Config) -> Result<Self, Self::Error> {
        static LOG_FORMATTER: &str = "[{d(%Y-%m-%d %H:%M:%S)}][{l}] {m}{n}";

        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(LOG_FORMATTER)))
            .build();

        let mut config_builder = log4rs::config::runtime::Config::builder();
        let mut root_builder = Root::builder();
        if cfg.enable_file {
            let file_path = Path::new(cfg.file_path.as_str());
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let file = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(LOG_FORMATTER)))
                .build(file_path)?;
            config_builder =
                config_builder.appender(Appender::builder().build("file", Box::new(file)));
            root_builder = root_builder.appender("file");
        }

        Ok(config_builder
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(
                root_builder
                    .appender("stdout")
                    .build(LevelFilter::from_str(cfg.level.as_str())?),
            )?)
    }
}
