// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::AppConfig;

fn main() {
    let app_config = AppConfig::from_env().unwrap_or_else(|err| {
        println!("Error: {err}");
        std::process::exit(1);
    });

    logger::init(
        app_config.log_level,
        &logger::generate_log_name(&app_config.name),
    )
    .unwrap_or_else(|err| {
        println!("Logger initialization failed. Error: {}", err);
        std::process::exit(1);
    });
}

pub mod config;
pub mod errors {
    pub mod env;
    pub mod log;
}
pub mod logger;
