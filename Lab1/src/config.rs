use crate::error::env::EnvError;
use dotenvy::dotenv;
use eframe::Theme;
use log::LevelFilter;
use std::env;
use std::str::FromStr;

pub struct AppConfig {
    pub name: String,
    pub log_level: LevelFilter,
    pub theme: Theme,
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

        // Loading theme
        let theme = env::var("THEME").map_err(|_| EnvError::ThemeNotLoaded)?;
        let theme = match theme.to_lowercase().as_str() {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            _ => return Err(EnvError::ThemeUndefined),
        };

        Ok(Self {
            name,
            log_level,
            theme,
        })
    }
}
