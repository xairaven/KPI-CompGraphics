use crate::config::AppConfig;

pub mod config;
pub mod errors;
pub mod logger;

fn main() {
    let app_config = AppConfig::from_env().unwrap_or_else(|err| {
        println!("Error: {err}");
        std::process::exit(1);
    });

    let log_file_name = logger::generate_log_name(app_config.name.clone());
    logger::init(app_config.log_level, &log_file_name).unwrap_or_else(|err| {
        println!("Logger initialization failed. Error: {}", err);
        std::process::exit(1);
    });
}
