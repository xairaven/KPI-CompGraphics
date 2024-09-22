use crate::error::env::EnvError;
use dotenvy::dotenv;
use log::LevelFilter;
use std::env;
use std::str::FromStr;

pub struct AppConfig {
    pub name: String,
    pub log_level: LevelFilter,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, EnvError> {
        // Loading .env from parent folder
        if dotenv().is_err() {
            return Err(EnvError::ConfigNotLoaded);
        }

        // Loading bot name
        let name = env::var("CRATE_NAME").map_err(|_| EnvError::NameNotLoaded)?;

        // Loading log level
        let log_level = env::var("LOG_LEVEL").map_err(|_| EnvError::LogLevelNotLoaded)?;
        let log_level =
            LevelFilter::from_str(&log_level).map_err(|_| EnvError::LogLevelUndefined)?;

        Ok(Self { name, log_level })
    }
}
